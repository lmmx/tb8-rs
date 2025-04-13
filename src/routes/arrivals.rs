use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use std::time::Instant;
use tracing::info;

use crate::error::AppResult;
use crate::models::{Prediction, Response};
use crate::routes::create_response;
use crate::tfl::TflClient;

pub fn arrivals_routes() -> Router {
    let tfl_client = Arc::new(TflClient::new());
    
    Router::new()
        .route("/arrivals-by-lines", get(get_arrivals_by_lines))
        .route("/arrivals-by-station", get(get_arrivals_by_station))
        .with_state(tfl_client)
}

#[derive(Debug, Deserialize)]
pub struct ArrivalsQuery {
    query: String,
    lines: Option<String>,
}

// Handler for /arrivals-by-lines
async fn get_arrivals_by_lines(
    State(tfl_client): State<Arc<TflClient>>,
    Query(params): Query<ArrivalsQuery>,
) -> AppResult<Json<Response<Prediction>>> {
    let start_time = Instant::now();
    let query = params.query;
    
    info!("Received query={}", query);
    
    // In the Python version, this parses comma-separated line IDs
    // We'll handle a single line ID for simplicity
    let lines: Vec<&str> = query.split(',').collect();
    let mut all_arrivals = Vec::new();
    
    for line in lines {
        let arrivals = tfl_client.get_arrivals_by_line(line.trim()).await?;
        all_arrivals.extend(arrivals);
    }
    
    let response = create_response(start_time, &query, all_arrivals);
    Ok(Json(response))
}

// Handler for /arrivals-by-station
async fn get_arrivals_by_station(
    State(tfl_client): State<Arc<TflClient>>,
    Query(params): Query<ArrivalsQuery>,
) -> AppResult<Json<Response<Prediction>>> {
    let start_time = Instant::now();
    let query = params.query;
    let lines = params.lines.unwrap_or_else(|| "tube".to_string());
    
    info!("Received query={}, lines={}", query, lines);
    
    // The station ID is in the query parameter
    let station_id = query.clone();
    
    // In the Python version, this handles multiple line IDs
    // For simplicity, we'll use the first line in the list
    let line_ids: Vec<&str> = lines.split(',').collect();
    let mut all_arrivals = Vec::new();
    
    for line_id in line_ids {
        let arrivals = tfl_client.get_arrivals_by_line_at_stop(line_id.trim(), &station_id).await?;
        all_arrivals.extend(arrivals);
    }
    
    let response = create_response(start_time, &station_id, all_arrivals);
    Ok(Json(response))
}
