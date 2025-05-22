#[cfg(feature = "server")]
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
#[cfg(feature = "server")]
use serde_json::json;
use std::fmt;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    BadRequest(String),
    InternalServerError(String),
    IoError(std::io::Error),
    SerializationError(serde_json::Error),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ApiError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            ApiError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
            ApiError::IoError(err) => write!(f, "IO error: {}", err),
            ApiError::SerializationError(err) => write!(f, "Serialization error: {}", err),
        }
    }
}

#[cfg(feature = "server")]
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::IoError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            ApiError::SerializationError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16()
        }));

        (status, body).into_response()
    }
}

impl From<std::io::Error> for ApiError {
    fn from(err: std::io::Error) -> Self {
        ApiError::IoError(err)
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        ApiError::SerializationError(err)
    }
}

impl std::error::Error for ApiError {}