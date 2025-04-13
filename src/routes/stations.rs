use axum::{
    extract::Query,
    routing::get,
    Json, Router,
};
// use polars::prelude::*;
use serde::Deserialize;
use std::time::Instant;
use tracing::info;

use crate::error::AppResult;
use crate::models::{Station, StationPoint, Response};
use crate::routes::create_response;

pub fn stations_routes() -> Router {
    Router::new()
        .route("/stations", get(get_stations))
        .route("/station-points", get(get_station_points))
        .route("/platforms", get(get_platforms))
}

#[derive(Debug, Deserialize)]
pub struct SqlQuery {
    query: Option<String>,
}

// Handler for /stations
// In a real implementation, this would load station data from a file or database
// For now, we'll return a mock response
async fn get_stations(
    Query(params): Query<SqlQuery>,
) -> AppResult<Json<Response<Station>>> {
    let start_time = Instant::now();
    let query = params.query.unwrap_or_else(|| "SELECT * FROM self;".to_string());
    
    info!("Received query={}", query);
    
    // In a real implementation, we would load the data from a file or database
    // For now, let's create some sample data
    let mut stations = Vec::new();
    
    stations.push(Station {
        station_unique_id: "940GZZLUASL".to_string(),
        station_name: "Arsenal".to_string(),
        fare_zones: Some("2".to_string()),
        hub_naptan_code: None,
        wifi: Some(true),
        outside_station_unique_id: Some("490G000ASL".to_string()),
        lat: Some(51.5586),
        lon: Some(-0.1059),
        lines: Some(vec!["piccadilly".to_string()]),
    });
    
    stations.push(Station {
        station_unique_id: "940GZZLUBKG".to_string(),
        station_name: "Barking".to_string(),
        fare_zones: Some("4".to_string()),
        hub_naptan_code: None,
        wifi: Some(true),
        outside_station_unique_id: Some("490G000BKG".to_string()),
        lat: Some(51.5396),
        lon: Some(0.0813),
        lines: Some(vec!["district".to_string(), "hammersmith-city".to_string(), "overground".to_string()]),
    });
    
    let response = create_response(start_time, &query, stations);
    Ok(Json(response))
}

// Handler for /station-points
async fn get_station_points(
    Query(params): Query<SqlQuery>,
) -> AppResult<Json<Response<StationPoint>>> {
    let start_time = Instant::now();
    let query = params.query.unwrap_or_else(|| "SELECT * FROM self;".to_string());
    
    info!("Received query={}", query);
    
    // Create mock station points
    let mut station_points = Vec::new();
    
    station_points.push(StationPoint {
        unique_id: "ASL-1".to_string(),
        station_unique_id: "940GZZLUASL".to_string(),
        area_name: "Arsenal Station".to_string(),
        area_id: 1,
        level: 0,
        lat: 51.5586,
        lon: -0.1059,
        friendly_name: "Arsenal Station Entrance".to_string(),
    });
    
    station_points.push(StationPoint {
        unique_id: "ASL-2".to_string(),
        station_unique_id: "940GZZLUASL".to_string(),
        area_name: "Arsenal Station Platform".to_string(),
        area_id: 2,
        level: -1,
        lat: 51.5587,
        lon: -0.1060,
        friendly_name: "Arsenal Station Platform".to_string(),
    });
    
    station_points.push(StationPoint {
        unique_id: "BKG-1".to_string(),
        station_unique_id: "940GZZLUBKG".to_string(),
        area_name: "Barking Station".to_string(),
        area_id: 3,
        level: 0,
        lat: 51.5396,
        lon: 0.0813,
        friendly_name: "Barking Station Entrance".to_string(),
    });
    
    let response = create_response(start_time, &query, station_points);
    Ok(Json(response))
}

// Handler for /platforms
async fn get_platforms(
    Query(params): Query<SqlQuery>,
) -> AppResult<Json<Response<serde_json::Value>>> {
    let start_time = Instant::now();
    let query = params.query.unwrap_or_else(|| "SELECT * FROM self;".to_string());
    
    info!("Received query={}", query);
    
    // For platforms, we'll just return a mock JSON response
    let mut platforms = Vec::new();
    
    // Create a few sample platforms
    let platform1 = serde_json::json!({
        "PlatformUniqueId": "ASL-P1",
        "StationUniqueId": "940GZZLUASL",
        "PlatformNumber": "1",
        "CardinalDirection": "NB",
        "PlatformNaptanCode": "940GZZLUASL1",
        "PlatformFriendlyName": "Northbound Platform 1",
        "IsCustomerFacing": true,
        "HasServiceInterchange": false
    });
    
    let platform2 = serde_json::json!({
        "PlatformUniqueId": "ASL-P2",
        "StationUniqueId": "940GZZLUASL",
        "PlatformNumber": "2",
        "CardinalDirection": "SB",
        "PlatformNaptanCode": "940GZZLUASL2",
        "PlatformFriendlyName": "Southbound Platform 2",
        "IsCustomerFacing": true,
        "HasServiceInterchange": false
    });
    
    platforms.push(platform1);
    platforms.push(platform2);
    
    let response = create_response(start_time, &query, platforms);
    Ok(Json(response))
}
