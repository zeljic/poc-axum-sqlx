use axum::extract::{Path, State};
use crate::app::AppState;
use crate::error::AppError;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{http, Router};
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{FromRow, Pool, Sqlite};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
	id: i64,
	name: String,
	email: String,
	created_at: String,
	updated_at: String,
	deleted_at: Option<String>,
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
	match sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1 LIMIT 1")
		.bind(id)
		.fetch_optional(&db)
		.await
		.map_err(|e| {
			AppError::InternalServerError(Some(Box::new(e)))
		})? {
		None => {
			Err(AppError::ServiceError(
				http::StatusCode::NOT_FOUND,
				"user_not_found".to_string(),
				None,
			))
		}
		Some(user) => {
			Ok(axum::Json(user))
		}
	}
}

pub async fn router(state: AppState) -> Result<Router, Box<dyn std::error::Error>> {
	let router = Router::new()
		.route("/", get(list))
		.route("/:id", get(item))
		.with_state(state);

	Ok(router)
}