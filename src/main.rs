mod error;
mod models;
mod routes;
mod tfl;

use axum::{
    http::Method,
    routing::get,
    Router,
    Json,
};
use serde_json::json;
use std::env;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber;

use crate::routes::{
    arrivals::arrivals_routes, disruption::disruption_routes, lines::lines_routes,
    stations::stations_routes,
};

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Load env vars (for API keys)
    dotenv::dotenv().ok();
    
    // Get the port from env or default to 4000
    let port = env::var("PORT").unwrap_or_else(|_| "4000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET])
        .allow_headers(Any);

    // Create the router with our routes
    let app = Router::new()
        .merge(lines_routes())
        .merge(stations_routes())
        .merge(arrivals_routes())
        .merge(disruption_routes())
        .route("/", get(root_handler))
        .layer(cors);

    info!("Starting server on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Add a handler function for the root route
async fn root_handler() -> Json<serde_json::Value> {
    Json(json!({ "🚨": "It's time for the tb8-rs!" }))
}