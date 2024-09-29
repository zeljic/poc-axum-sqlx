use std::error::Error;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::{json, Value};

pub enum AppError {
	InternalServerError(Option<Box<dyn Error>>),
	ServiceError(StatusCode, String, Option<Box<dyn Error>>),

	Json(StatusCode, Value, Option<Box<dyn Error>>),
}

impl IntoResponse for AppError {
	fn into_response(self) -> Response {
		match self {
			AppError::InternalServerError(e) => {
				if let Some(err) = e {
					tracing::error!("{:?}", err);
				} else {
					tracing::error!("Internal Server Error");
				}

				(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
			}
			AppError::ServiceError(code, error, e) => {
				AppError::Json(code, json!({ "error": error }), e).into_response()
			}
			AppError::Json(code, error, err) => {
				if let Some(err) = err {
					tracing::error!("{:?}", err);
				}

				(code, axum::Json(error)).into_response()
			}
		}
	}
}