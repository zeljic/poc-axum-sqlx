use crate::app::AppState;
use crate::error::AppError;
use crate::models::user::{DatabaseUser, ResponseUser};
use crate::routes::{validation_error_to_service_error, ValidJson};
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{http, Json, Router};
use serde_derive::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use validator::{Validate, ValidationError};

pub async fn list(State(db): State<Pool<Sqlite>>) -> Result<impl IntoResponse, AppError> {
    let users: Vec<ResponseUser> =
        sqlx::query_as::<_, DatabaseUser>("SELECT * FROM users ORDER BY id DESC")
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
        "SELECT * FROM users WHERE id = $1 AND deleted_at IS NOT NULL LIMIT 1",
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
    #[validate(length(min = 3, message = "name must be at least 3 characters"))]
    name: String,
    #[validate(email)]
    email: String,
}

/// Create a new user
pub async fn create(
    State(db): State<Pool<Sqlite>>,
    ValidJson(user): ValidJson<CreateUser>,
) -> Result<impl IntoResponse, AppError> {
    if let Ok(_) = sqlx::query("SELECT 1 FROM users WHERE email = $1")
        .bind(&user.email)
        .fetch_optional(&db)
        .await
        .map_err(|e| AppError::InternalServerError(Some(Box::new(e))))
    {
        return Err(validation_error_to_service_error(
            "email",
            ValidationError::new("email_exists").with_message("email_exists".into()),
        ));
    }

    if let Ok(_) = sqlx::query("SELECT 1 FROM users WHERE name = $1")
        .bind(&user.name)
        .fetch_optional(&db)
        .await
        .map_err(|e| AppError::InternalServerError(Some(Box::new(e))))
    {
        return Err(validation_error_to_service_error(
            "name",
            ValidationError::new("name_exists").with_message("name_exists".into()),
        ));
    }

    let user: ResponseUser = sqlx::query_as::<_, DatabaseUser>(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *",
    )
    .bind(user.name)
    .bind(user.email)
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
    if let None = sqlx::query("SELECT 1 FROM users WHERE id = $1 and deleted_at IS NULL")
        .bind(id)
        .fetch_optional(&db)
        .await
        .map_err(|e| AppError::InternalServerError(Some(Box::new(e))))?
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

pub async fn router(state: AppState) -> Result<Router, Box<dyn std::error::Error>> {
    let router = Router::new()
        .route("/", get(list))
        .route("/:id", get(item))
        .route("/", post(create))
        .route("/:id", delete(remove))
        .with_state(state);

    Ok(router)
}
