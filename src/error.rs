use std::error::Error;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

pub enum AppError {
	InternalServerError(Option<Box<dyn Error>>),
	ServiceError(StatusCode, String, Option<Box<dyn Error>>),
}

impl IntoResponse for AppError {
	fn into_response(self) -> Response {
		match self {
			AppError::InternalServerError(Some(e)) => {
				// TODO: Log the error
				println!("{:?}", e);

				(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
			}
			AppError::InternalServerError(None) => {
				(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
			}
			AppError::ServiceError(code, error, e) => {
				// TODO: Log the error
				match e {
					None => {
						println!("{:?}", error);
					}
					Some(e) => {
						println!("{:?}: {:?}", error, e);
					}
				}

				(code, axum::Json(json!({"error": error}))).into_response()
			}
		}
	}
}
