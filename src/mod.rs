pub mod lines;
pub mod stations;
pub mod arrivals;
pub mod disruption;

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::time::Instant;

use crate::models::{ErrorResponse, MetaData, Response};

// Helper function to create context metadata
pub fn create_metadata(start_time: Instant, query: &str) -> MetaData {
    let request_time = Utc::now();
    let response_time = Utc::now();
    let duration = start_time.elapsed();
    let latency_secs = duration.as_secs_f64();
    
    MetaData {
        request_time,
        response_time,
        response_latency: latency_secs,
        query: query.to_string(),
    }
}

// Helper function to create a successful response
pub fn create_response<T>(start_time: Instant, query: &str, results: Vec<T>) -> Response<T> {
    Response {
        context: create_metadata(start_time, query),
        success: true,
        results,
    }
}

// Helper function to create an error response
pub fn create_error_response(start_time: Instant, query: &str, error: String) -> ErrorResponse {
    ErrorResponse {
        context: create_metadata(start_time, query),
        success: false,
        error,
    }
}
