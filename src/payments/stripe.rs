// In cnctd_solana/src/payments/stripe.rs

use anyhow::{Result, Context, anyhow};
use base64::Engine;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnrampSessionResponse {
    pub id: String,
    pub client_secret: String,
    pub status: String,
    // Additional fields based on Stripe's response
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOnrampSessionRequest {
    pub transaction_details: TransactionDetails,
    pub customer_information: Option<CustomerInformation>,
    // Other optional configuration
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionDetails {
    pub destination_currency: String, // "usdc_solana"
    pub destination_network: String,  // "solana"
    pub destination_amount: Option<u64>,
    pub source_currency: Option<String>, // e.g., "usd"
    #[serde(rename = "destination_exchange_amount")]
    pub dest_exchange_amount: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerInformation {
    pub email: Option<String>,
    // Additional customer info if needed
}

pub struct Stripe {
    client: Client,
    api_key: String,
    base_url: String,
}

impl Stripe {
    pub fn new() -> Result<Self> {
        let api_key = env::var("STRIPE_API_KEY")
            .context("Failed to get STRIPE_API_KEY from environment")?;
        
        // Use test or live API based on the key
        let base_url = "https://api.stripe.com/v1".to_string();
        
        Ok(Self {
            client: Client::new(),
            api_key,
            base_url,
        })
    }
    
    pub async fn create_onramp_session(&self) -> Result<Value> {
        let url = format!("{}/crypto/onramp_sessions", self.base_url);

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "User-Agent",
            reqwest::header::HeaderValue::from_static("cnctd_studio"),
        );
        println!("api key: {:?}", self.api_key);
        headers.insert("Authorization", format!("Bearer {}", self.api_key).parse()?);

        let params = [
            ("wallet_addresses[solana]", "4vdBXSSMaD5yZqiGiD7ADffb1CiHphfXqx7ZqgudjihU"),
            ("source_currency", "usd"),
            ("destination_currency", "usdc"),
            ("destination_network", "solana"),
            ("destination_amount", "10")
        ];
    
        let res = self.client
            .post(url)
            .headers(headers)
            .form(&params)
            // .json(&body)
            .send()
            .await?;
        println!("response: {:?}", res);
        let res = res
            .json::<Value>()
            .await?;
    
        Ok(res)
        
        // // Base64 encode the API key with a trailing colon
        // let auth_value = format!("Basic {}", 
        //     base64::engine::general_purpose::STANDARD.encode(format!("{}:", self.api_key)));

            
        
        // let response = self.client
        //     .post(&url)
        //     // Use explicit Authorization header with Basic auth
        //     .header("Authorization", auth_value)
        //     .send()
        //     .await
        //     .context("Failed to send request to Stripe API")?;
        
        // if !response.status().is_success() {
        //     let error_text = response.text().await
        //         .context("Failed to get error response text")?;
        //     return Err(anyhow!("Onramp session creation failed: {}", error_text));
        // }
        
        // let session: OnrampSessionResponse = response.json().await
        //     .context("Failed to parse Stripe API response")?;
        
        // Ok(session)
    }
    
    // Additional methods for handling webhooks, checking session status, etc.
}