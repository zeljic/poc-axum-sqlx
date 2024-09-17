use axum::extract::{Path, State};
use crate::app::AppState;
use crate::error::AppError;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{http, Json, Router};
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Sqlite};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
	id: i64,
	name: String,
	email: String,
	created_at: DateTime<Utc>,
	updated_at: DateTime<Utc>,
	deleted_at: Option<DateTime<Utc>>,
}

pub async fn list(State(db): State<Pool<Sqlite>>) -> Result<impl IntoResponse, AppError> {
	let users = sqlx::query_as::<_, User>("SELECT * FROM users")
		.fetch_all(&db)
		.await
		.map_err(|e| {
			AppError::InternalServerError(Some(Box::new(e)))
		})?;


	Ok(axum::Json(users))
}

pub async fn item(State(db): State<Pool<Sqlite>>, Path(id): Path<i64>) -> Result<impl IntoResponse, AppError> {
	match sqlx::query_as::<_, User>("SELECT * FROM users1 WHERE id = $1 LIMIT 1")
		.bind(id)
		.fetch_optional(&db)
		.await
		.map_err(|e| {
			AppError::InternalServerError(Some(Box::new(e)))
		})? {
		None => {
			Err(AppError::ServiceError(
				http::StatusCode::NOT_FOUND,
				"not_found".to_string(),
				None,
			))
		}
		Some(user) => {
			Ok(axum::Json(user))
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUser {
	name: String,
	email: String,
}

/// Create a new user
///
pub async fn create(State(db): State<Pool<Sqlite>>, Json(user): Json<CreateUser>) -> Result<impl IntoResponse, AppError> {
	let user = sqlx::query_as::<_, User>(
		"INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *",
	)
		.bind(user.name)
		.bind(user.email)
		.fetch_one(&db)
		.await
		.map_err(|e| {
			AppError::InternalServerError(Some(Box::new(e)))
		})?;

	Ok(axum::Json(user))
}

pub async fn router(state: AppState) -> Result<Router, Box<dyn std::error::Error>> {
	let router = Router::new()
		.route("/", get(list))
		.route("/:id", get(item))
		.route("/", post(create))
		.with_state(state);

	Ok(router)
}