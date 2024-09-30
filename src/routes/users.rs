use crate::app::AppState;
use crate::error::AppError;
use crate::models::user::{DatabaseUser, ResponseUser};
use crate::routes::ValidJson;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post, put};
use axum::{http, Json, Router};
use serde_derive::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use validator::Validate;

pub async fn list(State(db): State<Pool<Sqlite>>) -> Result<impl IntoResponse, AppError> {
    let users: Vec<ResponseUser> =
        sqlx::query_as::<_, DatabaseUser>("SELECT * FROM users WHERE deleted_at IS NULL")
            .fetch_all(&db)
            .await
            .map_err(|e| AppError::InternalServerError(Some(Box::new(e))))?
            .into_iter()
            .map(|user| user.into())
            .collect();

    Ok(Json(users))
}

pub async fn item(
    State(db): State<Pool<Sqlite>>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    match sqlx::query_as::<_, DatabaseUser>(
        "SELECT * FROM users WHERE id = $1 AND deleted_at IS NULL LIMIT 1",
    )
    .bind(id)
    .fetch_optional(&db)
    .await
    .map_err(|e| AppError::InternalServerError(Some(Box::new(e))))?
    {
        None => Err(AppError::ServiceError(
            http::StatusCode::NOT_FOUND,
            "not_found".to_string(),
            None,
        )),
        Some(user) => Ok(Json(ResponseUser::from(user))),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateUser {
    #[validate(length(min = 3))]
    name: String,
    #[validate(email)]
    email: String,
    display_name: String,
}

/// Create a new user
pub async fn create(
    State(db): State<Pool<Sqlite>>,
    ValidJson(user): ValidJson<CreateUser>,
) -> Result<impl IntoResponse, AppError> {
    if let Ok(Some(_)) = sqlx::query("SELECT 1 FROM users WHERE email = $1")
        .bind(&user.email)
        .fetch_optional(&db)
        .await
        .map_err(|e| AppError::InternalServerError(Some(Box::new(e))))
    {
        return Err(AppError::Validation("email", "email_exists"));
    }

    if let Ok(Some(_)) = sqlx::query("SELECT 1 FROM users WHERE name = $1")
        .bind(&user.name)
        .fetch_optional(&db)
        .await
        .map_err(|e| AppError::InternalServerError(Some(Box::new(e))))
    {
        return Err(AppError::Validation("name", "name_exists"));
    }

    let user: ResponseUser = sqlx::query_as::<_, DatabaseUser>(
        "INSERT INTO users (name, email, display_name) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(user.name)
    .bind(user.email)
    .bind(user.display_name)
    .fetch_one(&db)
    .await
    .map_err(|e| AppError::InternalServerError(Some(Box::new(e))))
    .map(|user| user.into())?;

    Ok(Json(user))
}

pub async fn remove(
    State(db): State<Pool<Sqlite>>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    if sqlx::query("SELECT 1 FROM users WHERE id = $1 and deleted_at IS NULL")
        .bind(id)
        .fetch_optional(&db)
        .await
        .map_err(|e| AppError::InternalServerError(Some(Box::new(e))))?
        .is_none()
    {
        return Err(AppError::ServiceError(
            http::StatusCode::NOT_FOUND,
            "not_found".to_string(),
            None,
        ));
    }

    sqlx::query("UPDATE users SET deleted_at = CURRENT_TIMESTAMP WHERE id = $1")
        .bind(id)
        .execute(&db)
        .await
        .map_err(|e| AppError::InternalServerError(Some(Box::new(e))))?;

    Ok((http::StatusCode::NO_CONTENT, "").into_response())
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateUser {
    #[validate(length(min = 3))]
    name: String,
    #[validate(email)]
    email: String,
    display_name: String,
}

pub async fn update(
    State(db): State<Pool<Sqlite>>,
    Path(id): Path<i64>,
    ValidJson(user): ValidJson<UpdateUser>,
) -> Result<impl IntoResponse, AppError> {
    if let Ok(None) = sqlx::query("SELECT 1 FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(&db)
        .await
        .map_err(|e| AppError::InternalServerError(Some(Box::new(e))))
    {
        return Err(AppError::ServiceError(
            StatusCode::NOT_FOUND,
            "not_found".to_string(),
            None,
        ));
    }

    if let Ok(Some(_)) = sqlx::query("SELECT 1 FROM users WHERE id != $1 AND name = $2 LIMIT 1")
        .bind(id)
        .bind(&user.name)
        .fetch_optional(&db)
        .await
        .map_err(|e| AppError::InternalServerError(Some(Box::new(e))))
    {
        return Err(AppError::Validation("name", "name_user_by_another_user"));
    }

    if let Ok(Some(_)) = sqlx::query("SELECT 1 FROM users WHERE id != $1 AND email = $2 LIMIT 1")
        .bind(id)
        .bind(&user.email)
        .fetch_optional(&db)
        .await
        .map_err(|e| AppError::InternalServerError(Some(Box::new(e))))
    {
        return Err(AppError::Validation("email", "email_used_by_another_user"));
    }

    match sqlx::query_as::<_, DatabaseUser>(
        "UPDATE users SET name = $1, email = $2, display_name = $3 WHERE id = $4 RETURNING *",
    )
    .bind(&user.name)
    .bind(&user.email)
    .bind(&user.display_name)
    .bind(id)
    .fetch_optional(&db)
    .await
    .map_err(|e| AppError::InternalServerError(Some(Box::new(e))))?
    {
        None => Err(AppError::ServiceError(
            StatusCode::NOT_FOUND,
            "user_not_found".to_string(),
            None,
        )),
        Some(user) => Ok((StatusCode::OK, Json(ResponseUser::from(user)))),
    }
}

pub async fn router(state: AppState) -> Result<Router, Box<dyn std::error::Error>> {
    let router = Router::new()
        .route("/", get(list))
        .route("/:id", get(item))
        .route("/", post(create))
        .route("/:id", put(update))
        .route("/:id", delete(remove))
        .with_state(state);

    Ok(router)
}
