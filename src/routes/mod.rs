pub mod arrivals;
pub mod disruption;
pub mod stations;

use chrono::Utc;
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
#[allow(dead_code)]
pub fn create_response<T>(start_time: Instant, query: &str, results: Vec<T>) -> Response<T> {
    Response {
        context: create_metadata(start_time, query),
        success: true,
        results,
    }
}

// Helper function to create an error response
#[allow(dead_code)]
pub fn create_error_response(start_time: Instant, query: &str, error: String) -> ErrorResponse {
    ErrorResponse {
        context: create_metadata(start_time, query),
        success: false,
        error,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_response() {
        // Setup
        let start_time = Instant::now();
        let query = "test_query";
        let results = vec!["result1", "result2"];

        // Execute
        let response = create_response(start_time, query, results);

        // Verify
        assert!(response.success);
        assert_eq!(response.results, vec!["result1", "result2"]);
        assert_eq!(response.context.query, "test_query");
        // We can't test exact timing, but we can verify it's not negative
        assert!(response.context.response_latency >= 0.0);
    }
}
