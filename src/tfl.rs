use reqwest::{Client, Url};
use serde::de::DeserializeOwned;
use serde_json::Deserializer;
use serde_path_to_error::deserialize;
use std::env;
use tracing::debug;

use crate::error::{AppError, AppResult};
use crate::models::*;

const TFL_BASE_URL: &str = "https://api.tfl.gov.uk";

pub struct TflClient {
    client: Client,
    app_id: String,
    app_key: String,
}

impl TflClient {
    pub fn new() -> Self {
        let app_id = env::var("TFL_API_KEY_ID").unwrap_or_else(|_| "tb8-rs".to_string());
        let app_key = env::var("TFL_API_PRIMARY_ACCESS_KEY")
            .expect("TFL_API_PRIMARY_ACCESS_KEY environment variable must be set");

        Self {
            client: Client::new(),
            app_id,
            app_key,
        }
    }

    fn build_url(&self, path: &str) -> AppResult<Url> {
        let mut url = Url::parse(&format!("{}{}", TFL_BASE_URL, path))
            .map_err(|e| AppError::InternalError(format!("Failed to parse URL: {}", e)))?;

        url.query_pairs_mut()
            .append_pair("app_id", &self.app_id)
            .append_pair("app_key", &self.app_key);

        Ok(url)
    }

    async fn perform_request<T>(&self, path: &str) -> AppResult<T>
    where
        T: DeserializeOwned,
    {
        let url = self.build_url(path)?;

        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(AppError::TflApiError)?;

        self.deserialize_response(response).await
    }

    async fn deserialize_response<T>(&self, response: reqwest::Response) -> AppResult<T>
    where
        T: DeserializeOwned,
    {
        if !response.status().is_success() {
            let status = response.status();
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            let err_msg = format!("HTTP error {}: {}", status, text);
            return Err(AppError::InternalError(err_msg));
        }

        // Get the response body as bytes
        let bytes = response.bytes().await.map_err(AppError::TflApiError)?;

        // Use serde_path_to_error for deserialization
        let json_deserializer = &mut Deserializer::from_slice(&bytes);
        deserialize(json_deserializer).map_err(|e| AppError::DeserializationError {
            path: e.path().to_string(),
            message: e.to_string(),
            raw_data: Some(String::from_utf8_lossy(&bytes).to_string()),
        })
    }

    #[allow(dead_code)]
    pub async fn get_line_by_id(&self, line_id: &str) -> AppResult<Vec<Line>> {
        debug!("Fetching line by id: {}", line_id);
        self.perform_request(&format!("/Line/{}", line_id)).await
    }

    #[allow(dead_code)]
    pub async fn get_lines_by_mode(&self, mode: &str) -> AppResult<Vec<Line>> {
        debug!("Fetching lines by mode: {}", mode);
        self.perform_request(&format!("/Line/Mode/{}", mode)).await
    }

    #[allow(dead_code)]
    pub async fn get_arrivals_by_line(&self, line_id: &str) -> AppResult<Vec<Prediction>> {
        debug!("Fetching arrivals for line: {}", line_id);
        self.perform_request(&format!("/Line/{}/Arrivals", line_id))
            .await
    }

    pub async fn get_arrivals_by_line_at_stop(
        &self,
        line_id: &str,
        stop_id: &str,
    ) -> AppResult<Vec<Prediction>> {
        debug!("Fetching arrivals for line {} at stop {}", line_id, stop_id);
        self.perform_request(&format!("/Line/{}/Arrivals/{}", line_id, stop_id))
            .await
    }

    #[allow(dead_code)]
    pub async fn get_disruptions_by_line(&self, line_id: &str) -> AppResult<Vec<Disruption>> {
        debug!("Fetching disruptions for line: {}", line_id);
        self.perform_request(&format!("/Line/{}/Disruption", line_id))
            .await
    }

    pub async fn get_disruptions_by_mode(&self, mode: &str) -> AppResult<Vec<Disruption>> {
        debug!("Fetching disruptions for mode: {}", mode);
        self.perform_request(&format!("/Line/Mode/{}/Disruption", mode))
            .await
    }
}
