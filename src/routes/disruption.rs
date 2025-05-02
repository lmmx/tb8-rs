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
use crate::models::{Disruption, Response};
use crate::routes::create_response;
use crate::tfl::TflClient;

pub fn disruption_routes() -> Router {
    let tfl_client = Arc::new(TflClient::new());

    Router::new()
        .route("/disruption-by-modes", get(get_disruption_by_modes))
        .with_state(tfl_client)
}

#[derive(Debug, Deserialize)]
pub struct DisruptionQuery {
    query: String,
}

// Handler for /disruption-by-modes
async fn get_disruption_by_modes(
    State(tfl_client): State<Arc<TflClient>>,
    Query(params): Query<DisruptionQuery>,
) -> AppResult<Json<Response<Disruption>>> {
    let start_time = Instant::now();
    let query = params.query;

    info!("Received query={}", query);

    // Process comma-separated modes
    let modes: Vec<&str> = query.split(',').collect();
    let mut all_disruptions = Vec::new();

    // Validate that all modes are allowed
    let allowed_modes = ["tube", "overground", "dlr", "elizabeth-line"];
    for mode in &modes {
        if !allowed_modes.contains(mode) {
            return Err(crate::error::AppError::ParseError(format!(
                "Invalid mode: {}",
                mode
            )));
        }
    }

    // Fetch disruptions for each mode
    for mode in modes {
        let disruptions = tfl_client.get_disruptions_by_mode(mode.trim()).await?;
        all_disruptions.extend(disruptions);
    }

    let response = create_response(start_time, &query, all_disruptions);
    Ok(Json(response))
}
