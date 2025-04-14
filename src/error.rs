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

    #[error("Deserialization error at path '{path}': {message}")]
    DeserializationError {
        path: String,
        message: String,
        raw_data: Option<String>,
    },
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message, detail) = match self {
            AppError::TflApiError(err) => (StatusCode::BAD_GATEWAY, err.to_string(), None),
            AppError::ParseError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err, None),
            AppError::InternalError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err, None),
            AppError::NotFound(err) => (StatusCode::NOT_FOUND, err, None),
            AppError::DeserializationError { path, message, raw_data } => {
                let extracted_detail = raw_data.map(|data| {
                    // Try to get the relevant portion of the JSON
                    if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&data) {
                        // Try to extract the value at the path
                        let path_parts: Vec<&str> = path.split('.').collect();
                        extract_value_at_path(&json_value, &path_parts)
                            .map(|v| v.to_string())
                            .unwrap_or_else(|| "Could not extract problematic value".to_string())
                    } else {
                        "Invalid JSON".to_string()
                    }
                });

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("JSON deserialization failed at '{}': {}", path, message),
                    extracted_detail,
                )
            }
        };

        let mut response_json = json!({
            "success": false,
            "error": error_message,
        });

        if let Some(detail_value) = detail {
            response_json["detail"] = json!(detail_value);
        }

        let body = Json(response_json);
        (status, body).into_response()
    }
}

// Helper function to extract a value at a given path from JSON
fn extract_value_at_path<'a>(value: &'a serde_json::Value, path: &[&str]) -> Option<&'a serde_json::Value> {
    if path.is_empty() {
        return Some(value);
    }

    match value {
        serde_json::Value::Object(map) => {
            if let Some(next_value) = map.get(path[0]) {
                return extract_value_at_path(next_value, &path[1..]);
            }
        }
        serde_json::Value::Array(arr) => {
            // Handle array indexes in path like [0]
            if path[0].starts_with('[') && path[0].ends_with(']') {
                if let Ok(index) = path[0][1..path[0].len()-1].parse::<usize>() {
                    if index < arr.len() {
                        return extract_value_at_path(&arr[index], &path[1..]);
                    }
                }
            }
        }
        _ => return None,
    }
    None
}

pub type AppResult<T> = Result<T, AppError>;
