use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tracing::info;

use crate::error::AppResult;
use crate::models::{Line, Response};
use crate::routes::create_response;
use crate::tfl::TflClient;

pub fn lines_routes() -> Router {
    let tfl_client = Arc::new(TflClient::new());
    
    Router::new()
        .route("/lines", get(get_lines))
        .route("/lines-by-station", get(get_lines_by_station))
        .route("/lines-by-mode/:mode", get(get_lines_by_mode))
        .route("/lines/:id", get(get_line_by_id))
        .with_state(tfl_client)
}

#[derive(Debug, Deserialize)]
pub struct SqlQuery {
    query: Option<String>,
}

// Handler for /lines
async fn get_lines(
    State(tfl_client): State<Arc<TflClient>>,
    Query(params): Query<SqlQuery>,
) -> AppResult<Json<Response<Line>>> {
    let start_time = Instant::now();
    let query = params.query.unwrap_or_else(|| "SELECT * FROM self;".to_string());
    
    info!("Received query={}", query);
    
    // In a real implementation, we'd process the SQL query
    // For now, we just fetch all lines
    let lines = tfl_client.get_lines().await?;
    
    let response = create_response(start_time, &query, lines);
    Ok(Json(response))
}

// Handler for /lines-by-station
// In the Python version, this uses polars dataframes
// For a simple implementation, we'll mock this endpoint
async fn get_lines_by_station(
    State(tfl_client): State<Arc<TflClient>>,
    Query(params): Query<SqlQuery>,
) -> AppResult<Json<Response<HashMap<String, serde_json::Value>>>> {
    let start_time = Instant::now();
    let query = params.query.unwrap_or_else(|| "SELECT * FROM self;".to_string());
    
    info!("Received query={}", query);
    
    // In a full implementation, this would come from actual data
    // For now, return a mock response
    let mut results = Vec::new();
    
    // Create a few sample stations with their lines
    let mut station1 = HashMap::new();
    station1.insert("StationUniqueId".to_string(), serde_json::Value::String("940GZZLUASL".to_string()));
    station1.insert("StationName".to_string(), serde_json::Value::String("Arsenal".to_string()));
    station1.insert("Lines".to_string(), serde_json::json!(["piccadilly"]));

    let mut station2 = HashMap::new();
    station2.insert("StationUniqueId".to_string(), serde_json::Value::String("940GZZLUBKG".to_string()));
    station2.insert("StationName".to_string(), serde_json::Value::String("Barking".to_string()));
    station2.insert("Lines".to_string(), serde_json::json!(["district", "hammersmith-city", "overground"]));
    
    results.push(station1);
    results.push(station2);
    
    let response = create_response(start_time, &query, results);
    Ok(Json(response))
}

// Handler for /lines/:id
async fn get_line_by_id(
    State(tfl_client): State<Arc<TflClient>>,
    Path(id): Path<String>,
    Query(params): Query<SqlQuery>,
) -> AppResult<Json<Response<Line>>> {
    let start_time = Instant::now();
    let query = format!("id={}", id);
    
    info!("Received {}", query);
    
    let lines = tfl_client.get_line_by_id(&id).await?;
    
    let response = create_response(start_time, &query, lines);
    Ok(Json(response))
}

// Handler for /lines-by-mode/:mode
async fn get_lines_by_mode(
    State(tfl_client): State<Arc<TflClient>>,
    Path(mode): Path<String>,
    Query(params): Query<SqlQuery>,
) -> AppResult<Json<Response<Line>>> {
    let start_time = Instant::now();
    let query = format!("mode={}", mode);
    
    info!("Received {}", query);
    
    let lines = tfl_client.get_lines_by_mode(&mode).await?;
    
    let response = create_response(start_time, &query, lines);
    Ok(Json(response))
}
