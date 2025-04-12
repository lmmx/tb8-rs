use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("TfL API request failed: {0}")]
    TflApiError(#[from] reqwest::Error),
    
    #[error("Failed to parse TfL response: {0}")]
    ParseError(String),
    
    #[error("Internal server error: {0}")]
    InternalError(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::TflApiError(err) => (StatusCode::BAD_GATEWAY, err.to_string()),
            AppError::ParseError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),
            AppError::InternalError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),
            AppError::NotFound(err) => (StatusCode::NOT_FOUND, err),
        };

        let body = Json(json!({
            "success": false,
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;