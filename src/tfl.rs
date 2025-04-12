use std::env;
use reqwest::{Client, Url};
use tracing::{debug, error};

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

    pub async fn get_lines(&self) -> AppResult<Vec<Line>> {
        debug!("Fetching all lines");
        let url = self.build_url("/Line")?;
        
        let response = self.client.get(url)
            .send()
            .await
            .map_err(AppError::TflApiError)?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            let err_msg = format!("HTTP error {}: {}", status, text);
            return Err(AppError::InternalError(err_msg));
        }
        
        let lines: Vec<Line> = response.json()
            .await
            .map_err(AppError::TflApiError)?;
            
        Ok(lines)
    }
    
    pub async fn get_line_by_id(&self, line_id: &str) -> AppResult<Vec<Line>> {
        debug!("Fetching line by id: {}", line_id);
        let url = self.build_url(&format!("/Line/{}", line_id))?;
        
        let response = self.client.get(url)
            .send()
            .await
            .map_err(AppError::TflApiError)?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            let err_msg = format!("HTTP error {}: {}", status, text);
            return Err(AppError::InternalError(err_msg));
        }
        
        let lines: Vec<Line> = response.json()
            .await
            .map_err(AppError::TflApiError)?;
            
        Ok(lines)
    }
    
    pub async fn get_lines_by_mode(&self, mode: &str) -> AppResult<Vec<Line>> {
        debug!("Fetching lines by mode: {}", mode);
        let url = self.build_url(&format!("/Line/Mode/{}", mode))?;
        
        let response = self.client.get(url)
            .send()
            .await
            .map_err(AppError::TflApiError)?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            let err_msg = format!("HTTP error {}: {}", status, text);
            return Err(AppError::InternalError(err_msg));
        }
        
        let lines: Vec<Line> = response.json()
            .await
            .map_err(AppError::TflApiError)?;
            
        Ok(lines)
    }
    
    pub async fn get_arrivals_by_line(&self, line_id: &str) -> AppResult<Vec<Prediction>> {
        debug!("Fetching arrivals for line: {}", line_id);
        let url = self.build_url(&format!("/Line/{}/Arrivals", line_id))?;
        
        let response = self.client.get(url)
            .send()
            .await
            .map_err(AppError::TflApiError)?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            let err_msg = format!("HTTP error {}: {}", status, text);
            return Err(AppError::InternalError(err_msg));
        }
        
        let predictions: Vec<Prediction> = response.json()
            .await
            .map_err(AppError::TflApiError)?;
            
        Ok(predictions)
    }
    
    pub async fn get_arrivals_by_line_at_stop(&self, line_id: &str, stop_id: &str) -> AppResult<Vec<Prediction>> {
        debug!("Fetching arrivals for line {} at stop {}", line_id, stop_id);
        let url = self.build_url(&format!("/Line/{}/Arrivals/{}", line_id, stop_id))?;
        
        let response = self.client.get(url)
            .send()
            .await
            .map_err(AppError::TflApiError)?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            let err_msg = format!("HTTP error {}: {}", status, text);
            return Err(AppError::InternalError(err_msg));
        }
        
        let predictions: Vec<Prediction> = response.json()
            .await
            .map_err(AppError::TflApiError)?;
            
        Ok(predictions)
    }
    
    pub async fn get_disruptions_by_line(&self, line_id: &str) -> AppResult<Vec<Disruption>> {
        debug!("Fetching disruptions for line: {}", line_id);
        let url = self.build_url(&format!("/Line/{}/Disruption", line_id))?;
        
        let response = self.client.get(url)
            .send()
            .await
            .map_err(AppError::TflApiError)?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            let err_msg = format!("HTTP error {}: {}", status, text);
            return Err(AppError::InternalError(err_msg));
        }
        
        let disruptions: Vec<Disruption> = response.json()
            .await
            .map_err(AppError::TflApiError)?;
            
        Ok(disruptions)
    }
    
    pub async fn get_disruptions_by_mode(&self, mode: &str) -> AppResult<Vec<Disruption>> {
        debug!("Fetching disruptions for mode: {}", mode);
        let url = self.build_url(&format!("/Line/Mode/{}/Disruption", mode))?;
        
        let response = self.client.get(url)
            .send()
            .await
            .map_err(AppError::TflApiError)?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            let err_msg = format!("HTTP error {}: {}", status, text);
            return Err(AppError::InternalError(err_msg));
        }
        
        let disruptions: Vec<Disruption> = response.json()
            .await
            .map_err(AppError::TflApiError)?;
            
        Ok(disruptions)
    }
}
