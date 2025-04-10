// In cnctd_solana/src/payments/circle.rs

use anyhow::{Result, Context, anyhow};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutSessionResponse {
    pub data: CheckoutSessionData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutSessionData {
    pub id: String,
    #[serde(rename = "type")]
    pub session_type: String,
    pub success_url: Option<String>,
    pub client_token: String,
    pub status: String,
    pub expires_on: String,
    pub create_date: String,
    pub update_date: String,
    pub amount: Amount,
    pub amount_paid: Option<Amount>,
    pub payment_ids: Option<Vec<String>>,
    pub payment_intent_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Amount {
    pub amount: String,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCheckoutSessionRequest {
    pub amount: Amount,
    #[serde(rename = "successUrl", skip_serializing_if = "Option::is_none")]
    pub success_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

pub struct Circle {
    client: Client,
    api_key: String,
    base_url: String,
}

impl Circle {
    pub fn new() -> Result<Self> {
        let api_key = env::var("CIRCLE_API_KEY")
            .context("Failed to get CIRCLE_API_KEY from environment")?;
        
        // Default to sandbox URL, can be overridden with env var
        let base_url = env::var("CIRCLE_API_URL")
            .unwrap_or_else(|_| "https://api-sandbox.circle.com".to_string());
        
        Ok(Self {
            client: Client::new(),
            api_key,
            base_url,
        })
    }
    
    pub async fn create_checkout_session(&self, request: CreateCheckoutSessionRequest) -> Result<CheckoutSessionResponse> {
        let url = format!("{}/v1/checkoutSessions", self.base_url);
        
        let response = self.client
            .post(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Circle API")?;
        
        if !response.status().is_success() {
            let error_text = response.text().await
                .context("Failed to get error response text")?;
            return Err(anyhow!("Checkout session creation failed: {}", error_text));
        }
        
        let session: CheckoutSessionResponse = response.json().await
            .context("Failed to parse Circle API response")?;
        
        Ok(session)
    }
}