use std::ops::Deref;
use axum::extract::{FromRequest, Request};
use axum::{http, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::{Validate, ValidationError, ValidationErrors};
use crate::app::AppState;
use crate::error::AppError;

mod users;

pub async fn router(app_state: AppState) -> Result<Router, Box<dyn std::error::Error>> {
	let router = Router::new()
		.nest("/users", users::router(AppState::clone(&app_state)).await?);

	Ok(router)
}

#[derive(Debug)]
pub struct ValidJson<T>(pub T)
where
	T: Serialize + for<'de> serde::Deserialize<'de> + Validate;

impl<T> Deref for ValidJson<T>
where
	T: Serialize + for<'de> serde::Deserialize<'de> + Validate,
{
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}


#[axum::async_trait]
impl<S, T> FromRequest<S> for ValidJson<T>
where
	S: Send + Sync,
	T: Serialize + for<'de> Deserialize<'de> + Validate,
{
	type Rejection = AppError;

	async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
		let payload: T = axum::Json::from_request(req, state)
			.await
			.map_err(|e| AppError::ServiceError(http::StatusCode::BAD_REQUEST, "invalid_json".to_string(), Some(Box::new(e))))
			.map(|json| json.0)?;

		payload.validate().map_err(validation_errors_to_service_error)?;

		Ok(ValidJson(payload))
	}
}

pub fn validation_errors_to_service_error(errors: ValidationErrors) -> AppError {
	match serde_json::to_value(errors.into_errors()) {
		Ok(errors) => AppError::Json(http::StatusCode::BAD_REQUEST, json!({
			"error": "validation_error",
			"errors": errors
		}), None),
		Err(_) => AppError::InternalServerError(None),
	}
}

pub fn validation_error_to_service_error(field: &'static str, error: ValidationError) -> AppError {
	let mut errors = ValidationErrors::new();

	errors.add(field, error);

	validation_errors_to_service_error(errors)
}