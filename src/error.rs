use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::{json, Value};
use std::error::Error;
use validator::{ValidationError, ValidationErrors};

pub enum AppError {
    InternalServerError(Option<Box<dyn Error>>),
    ServiceError(StatusCode, String, Option<Box<dyn Error>>),

    Json(StatusCode, Value, Option<Box<dyn Error>>),
    ValidationErrors(ValidationErrors),
    Validation(&'static str, &'static str),
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
            AppError::ValidationErrors(errors) => AppError::from(errors).into_response(),
            AppError::Validation(field, message) => {
                let mut validation_errors = ValidationErrors::new();

                validation_errors.add(
                    field,
                    ValidationError::new(message).with_message(message.into()),
                );

                AppError::ValidationErrors(validation_errors).into_response()
            }
        }
    }
}

impl From<ValidationErrors> for AppError {
    fn from(errors: ValidationErrors) -> Self {
        match serde_json::to_value(errors.into_errors()) {
            Ok(errors) => AppError::Json(
                StatusCode::BAD_REQUEST,
                json!({
                    "error": "validation",
                    "errors": errors
                }),
                None,
            ),
            Err(_) => AppError::InternalServerError(None),
        }
    }
}
