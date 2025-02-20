// use std::{str::FromStr, time::{Duration, Instant}};

// use chrono::{DateTime, TimeZone, Utc};
// use anyhow::anyhow;
// use futures::StreamExt;
// use kebtech_utils::*;
// use orca_pools_ipc_types::{request::Request, response::Response};
// use rpc_url::RpcUrl;
// use serde::{Deserialize, Serialize};
// use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig, rpc_response::{RpcResponseContext, RpcSimulateTransactionResult}};
// use solana_sdk::{commitment_config::CommitmentLevel, hash::Hash, instruction::Instruction, pubkey::Pubkey, signature::{Keypair, Signature}, signer::Signer, transaction::{self, Transaction, VersionedTransaction}};
// use solana_transaction_status::TransactionStatus;
// use tokio::time::{sleep, timeout};

// use crate::wallet::Wallet;

// pub mod rpc_url;

pub mod client;

// #[derive(Debug, Serialize, Deserialize, Clone, Copy)]
// pub enum ComputeUnitLimit {
//     Default,         // Default compute unit limit (200,000 CUs)
//     HighUsage,       // Higher compute unit limit for more complex actions
//     Max,             // Maximum compute unit limit (1.4 million CUs)
// }

// #[derive(Debug, Serialize, Deserialize, Clone, Copy)]
// pub enum PriorityFee {
//     None,            // No priority fee
//     Low,             // Low priority fee (e.g., 500 micro-lamports per CU)
//     Medium,          // Medium priority fee (e.g., 1,500 micro-lamports per CU)
//     High,            // High priority fee (e.g., 10,000+ micro-lamports per CU)
//     Highest,       // Highest priority fee (e.g., 10,000+ micro-lamports per CU)
//     Custom(u64),     // Custom priority fee (e.g., 5,000 micro-lamports per CU)
// }

// impl ComputeUnitLimit {
//     pub fn to_limit(&self) -> Option<u32> {
//         match self {
//             ComputeUnitLimit::Default => None,          // Default is 200,000
//             ComputeUnitLimit::HighUsage => Some(500_000), // High usage: 500,000 CU
//             ComputeUnitLimit::Max => Some(1_400_000),    // Max limit: 1.4M CU
//         }
//     }
// }

// impl PriorityFee {
//     pub fn to_micro_lamports(&self) -> u64 {
//         match self {
//             PriorityFee::None => 0,                     // No priority fee
//             PriorityFee::Low => 500,                    // 500 micro-lamports per CU
//             PriorityFee::Medium => 1_500,               // 1,500 micro-lamports per CU
//             PriorityFee::High => 5_000_000,                // 10,000 micro-lamports per CU
//             PriorityFee::Highest => 10_000_000,              // Highest priority fee (e.g., 10,000+ micro-lamports per CU)
//             PriorityFee::Custom(fee) => *fee,           // Custom fee
//         }
//     }
// }

// pub trait DomainExtractor {
//     fn domain(&self) -> String;
// }

// impl DomainExtractor for str {
//     fn domain(&self) -> String {
//         self.split('/')
//             .nth(2) // Get the 3rd segment, where the domain is typically located
//             .map(|s| s.to_string()) // Convert it to a String
//             .unwrap_or_default() // Handle the Option by providing a default value
//     }
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub enum RpcMode {
//     Failover(Vec<String>),
//     Concurrent(Vec<String>),
// }

// impl RpcMode {
//     pub fn single(rpc_url: String) -> Self {
//         RpcMode::Failover(vec![rpc_url])
//     }

//     pub fn fast() -> Self {
//         RpcMode::Concurrent(RpcUrl::speed_priority())
//     }

//     pub fn conservative() -> Self {
//         RpcMode::Failover(RpcUrl::volume_priority())
//     }
// }

// pub struct Rpc;

// impl Rpc {
//     pub async fn call<T, F>(
//         client_call: F,
//         timeout_ms: Option<u64>,
//         mode: RpcMode,
//     ) -> anyhow::Result<T>
//     where
//         F: Fn(RpcClient) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<T>> + Send>>
//             + Sync
//             + Clone
//             + Send
//             + 'static,
//         T: Send 
//             + 'static
//             + std::fmt::Debug,
//     {
//         match mode {
//             RpcMode::Failover(rpc_urls) => {

//                 for (i, url) in rpc_urls.iter().enumerate() {
//                     let client = RpcClient::new(url.clone());
//                     let start = Utc::now();
    
//                     let timeout_duration = if timeout_ms.is_some() {
//                         timeout_ms.map(Duration::from_millis)
//                     } else {
//                         Some(Duration::from_secs(20))
//                     };
    
//                     let result = match timeout_duration {
//                         Some(duration) => timeout(duration, client_call(client)).await,
//                         None => Ok(client_call(client).await),
//                     };
    
//                     match result {
//                         Ok(Ok(data)) => {
//                             let elapsed = Utc::now().signed_duration_since(start).num_milliseconds();
//                             println!("Success from RPC Domain: {:?}, Time: {}ms", url.domain(), elapsed);
//                             return Ok(data);
//                         }
//                         Ok(Err(err)) => {
//                             let elapsed =
//                                 Utc::now().signed_duration_since(start).num_milliseconds();
//                             red!(
//                                 "Failed call from RPC Domain: {}, Time: {}ms, Error: {:?}",
//                                 url.domain(),
//                                 elapsed,
//                                 err
//                             );
//                         }
//                         Err(_) => {
//                             let elapsed =
//                                 Utc::now().signed_duration_since(start).num_milliseconds();
//                             red!(
//                                 "Timeout! RPC Domain: {}, Time: {}ms exceeded timeout of {:?}",
//                                 url.domain(),
//                                 elapsed,
//                                 timeout_duration.unwrap_or_default()
//                             );
//                         }
//                     }
//                 }
    
//                 Err(anyhow::anyhow!("All RPC calls failed"))
//             }
    
//             RpcMode::Concurrent(rpc_urls) => {
//                 let futures = rpc_urls.into_iter().map(|url| {
//                     let client = RpcClient::new(url.clone());
//                     let client_call = client_call.clone();
//                     let timeout_duration = timeout_ms.map(Duration::from_millis);
    
//                     async move {
//                         let start = Utc::now();
//                         let result = match timeout_duration {
//                             Some(duration) => timeout(duration, client_call(client)).await,
//                             None => Ok(client_call(client).await),
//                         };
    
//                         match result {
//                             Ok(Ok(data)) => {
//                                 let elapsed =
//                                     Utc::now().signed_duration_since(start).num_milliseconds();
//                                 // println!("Success from RPC Domain: {:?}, Time: {}ms", url.domain(), elapsed);
//                                 Ok(data)
//                             }
//                             Ok(Err(err)) => {
//                                 let elapsed =
//                                     Utc::now().signed_duration_since(start).num_milliseconds();
//                                 red!(
//                                     "Failed call from RPC Domain: {}, Time: {}ms, Error: {:?}",
//                                     url.domain(),
//                                     elapsed,
//                                     err
//                                 );
//                                 Err(err)
//                             }
//                             Err(_) => {
//                                 let elapsed =
//                                     Utc::now().signed_duration_since(start).num_milliseconds();
//                                 red!(
//                                     "Timeout! RPC Domain: {}, Time: {}ms exceeded timeout of {:?}",
//                                     url.domain(),
//                                     elapsed,
//                                     timeout_duration.unwrap_or_default()
//                                 );
//                                 Err(anyhow::anyhow!("Timeout"))
//                             }
//                         }
//                     }
//                 });
    
//                 let mut tasks = futures::stream::FuturesUnordered::new();

//                 for future in futures {
//                     tasks.push(future);
//                 }
                
//                 while let Some(result) = tasks.next().await {
//                     tracing::info!("Received result: {:?}", result);
//                     if let Ok(data) = result {
//                         println!("RPC result data: {:?}", data);
//                         return Ok(data);
//                     }
//                 }
                
//                 Err(anyhow::anyhow!("All RPC calls failed"))
//             }
//         }
//     }
     
//     pub async fn call_orca<F>(
//         mode: RpcMode,
//         request_builder: F,
//         timeout_ms: Option<u64>
//     ) -> anyhow::Result<Response>
//     where
//         F: Fn(String) -> Request + Clone + Sync + Send + 'static,
//     {
//         match mode {
//             RpcMode::Failover(rpc_urls) => {
//                 for (i, url) in rpc_urls.iter().enumerate() {
//                     let start = Utc::now();
//                     let timeout_duration = if timeout_ms.is_some() {
//                         timeout_ms.map(Duration::from_millis)
//                     } else {
//                         Some(Duration::from_secs(20))
//                     };

//                     let request = request_builder(url.clone());

//                     let result = match timeout_duration {
//                         Some(duration) => timeout(duration, request.send()).await,
//                         None => Ok(request.send().await),
//                     };

//                     match result {
//                         Ok(Ok(response)) => {
//                             // println!(
//                             //     "Success from RPC Domain: {}, Time: {}ms",
//                             //     url.domain(),
//                             //     Utc::now().signed_duration_since(start).num_milliseconds(),
//                             // );
//                             return Ok(response);
//                         }
//                         Ok(Err(err)) => {
//                             red!(
//                                 "Failed call from RPC Domain: {}, Error: {:?}",
//                                 url.domain(),
//                                 err
//                             );
//                         }
//                         Err(_) => {
//                             red!(
//                                 "Timeout! RPC Domain: {}, Time exceeded timeout of 10s",
//                                 url.domain()
//                             );
//                         }
//                     }
//                 }

//                 Err(anyhow::anyhow!("All RPC calls failed"))
//             }

//             RpcMode::Concurrent(rpc_urls) => {
//                 let futures = rpc_urls.into_iter().map(|url| {
//                     let request = request_builder(url.clone());
//                     async move {
//                         let start = Utc::now();
//                         let result = timeout(Duration::from_secs(120), request.send()).await;
//                         match result {
//                             Ok(Ok(response)) => {
//                                 // println!(
//                                 //     "Success from RPC Domain: {}, Time: {}ms",
//                                 //     url.domain(),
//                                 //     Utc::now().signed_duration_since(start).num_milliseconds(),
//                                 // );
//                                 Ok(response)
//                             }
//                             Ok(Err(err)) => {
//                                 red!(
//                                     "Failed call from RPC Domain: {}, Error: {:?}",
//                                     url.domain(),
//                                     err
//                                 );
//                                 Err(anyhow::anyhow!("RPC call failed"))
//                             }
//                             Err(_) => {
//                                 red!(
//                                     "Timeout! RPC Domain: {}, Time exceeded timeout of 10s",
//                                     url.domain()
//                                 );
//                                 Err(anyhow::anyhow!("Timeout"))
//                             }
//                         }
//                     }
//                 });

//                 let mut tasks = futures::stream::FuturesUnordered::new();

//                 for future in futures {
//                     tasks.push(future);
//                 }
                
//                 while let Some(result) = tasks.next().await {
//                     if let Ok(data) = result {
//                         println!("RPC result data: {:?}", data);
//                         return Ok(data);
//                     }
//                 }
                
                
//                 Err(anyhow::anyhow!("All RPC calls failed"))
//             }
//         }
//     }

//     pub async fn get_latest_blockhash(rpc_mode: RpcMode, timeout_ms: Option<u64>) -> anyhow::Result<Hash> {
//         println!("Fetching latest blockhash");
//         let response = Rpc::call(
//             move |client| {
//                 Box::pin(async move {
//                     client.get_latest_blockhash().await.map_err(|e| e.into())
//                 })
//             },
//             timeout_ms,
//             rpc_mode,
//         ).await?;

//         Ok(response)
//     }

//     pub async fn send_and_confirm_transaction_with_config(
//         rpc_mode: RpcMode,
//         instructions: Vec<Instruction>,
//         signers: Vec<Box<dyn Signer>>,
//         timeout_ms: Option<u64>,
//     ) -> anyhow::Result<Signature> {
//         // Resolve the latest blockhash
//         let start = Utc::now();
//         blue!("Fetching latest blockhash");
//         let recent_blockhash = Rpc::get_latest_blockhash(rpc_mode.clone(), timeout_ms).await?;
    
//         green!(
//             "Fetched latest blockhash in {}ms",
//             start.signed_duration_since(Utc::now()).num_milliseconds()
//         );
    
//         // Build the transaction
//         let message = solana_sdk::message::Message::new(&instructions, Some(&signers[0].pubkey()));
//         let transaction = Transaction::new(&signers, message, recent_blockhash);
    
//         // Define transaction config
//         let transaction_config = RpcSendTransactionConfig {
//             skip_preflight: true,
//             preflight_commitment: Some(CommitmentLevel::Confirmed),
//             max_retries: Some(0),
//             ..Default::default()
//         };
    
//         // Send the transaction
//         let send_transaction_result = Rpc::call(
//             move |client| {
//                 let transaction = transaction.clone();
//                 yellow!("Sending transaction with {}...", client.url().domain());
//                 let config = transaction_config.clone();
//                 Box::pin(async move {
//                     let signature = client
//                         .send_transaction_with_config(&transaction, config)
//                         .await?;
//                     sleep(Duration::from_secs(2)).await;
//                     let statuses = client.get_signature_statuses(&[signature]).await?;
//                     Ok((signature, statuses.value))
//                 })
//             },
//             timeout_ms,
//             rpc_mode.clone(),
//         )
//         .await;
    
//         // Handle the transaction result
//         let (signature, statuses) = send_transaction_result?;
//         if let Some(Some(status)) = statuses.get(0) {
//             let status = status.clone();
//             println!("Transaction status: {:?}", status);
//             if status.confirmation_status.is_some() && status.err.is_none() {
//                 green!("Transaction confirmed with signature: {}", signature);
//                 return Ok(signature);
//             } else if let Some(err) = status.err {
//                 return Err(anyhow::anyhow!(format!("Transaction error: {:?}", err)));
//             }
//         }
    
//         Err(anyhow::anyhow!("Transaction failed to confirm"))
//     }
    
    
//     pub async fn send_and_confirm_transaction<F, Fut>(
//         rpc_mode: RpcMode,
//         instructions: Vec<Instruction>,
//         signers: Vec<Box<dyn Signer>>,
//         timeout_ms: Option<u64>,
//         last_check_function: F, // Now always required, but can be a default function
//     ) -> anyhow::Result<Signature>
//     where
//         F: Fn() -> Fut + Send + Sync + 'static,
//         Fut: std::future::Future<Output = bool> + Send,
//     {
//         let start = Instant::now();
//         let recent_blockhash = Rpc::get_latest_blockhash(rpc_mode.clone(), timeout_ms).await?;
//         blue!("Fetched latest blockhash in {}ms", start.elapsed().as_millis());
    
//         let message = solana_sdk::message::Message::new(&instructions, Some(&signers[0].pubkey()));
//         let transaction = Transaction::new(&signers, message, recent_blockhash);
    
//         let transaction_clone = transaction.clone();
    
//         if !last_check_function().await { // Await the async function
//             return Err(anyhow::anyhow!("Last check function failed"));
//         }
    
//         let start = Instant::now();
//         let signature = Rpc::call(
//             move |client| {
//                 yellow!("Sending transaction with {}...", client.url().domain());
//                 let transaction = transaction_clone.clone();
//                 Box::pin(async move {
//                     client.send_and_confirm_transaction(&transaction).await.map_err(|e| e.into())
//                 })
//             },
//             timeout_ms,
//             rpc_mode.clone(),
//         ).await?;
        
//         green!("sent transaction in {}ms", start.elapsed().as_millis());
    
//         Ok(signature)
//     }
    

//     pub async fn send_raw_transaction(
//         rpc_mode: RpcMode,
//         keypair: Keypair,
//         raw_transaction: Vec<u8>, 
//         timeout_ms: Option<u64>,
//     ) -> anyhow::Result<Signature> {
//         println!("Deserializing raw transaction");
//         let transaction: VersionedTransaction = bincode::deserialize(&raw_transaction)?;
//         println!("Deserialized transaction: {:?}", transaction);
        
//         let signed_versioned_transaction = VersionedTransaction::try_new(transaction.message, &[&keypair])?;
        
//         let transaction_clone = signed_versioned_transaction.clone();
//         let signature = Rpc::call(
//             move |client| {
//                 let transaction = transaction_clone.clone();
//                 Box::pin(async move {
//                     client.send_and_confirm_transaction(&transaction).await.map_err(|e| e.into())
//                 })
//             },
//             timeout_ms,
//             rpc_mode.clone(),
//         ).await?;
    
//         Ok(signature)
//     }
    
    
//     pub async fn get_account_creation_date(
//         rpc_mode: RpcMode,
//         address: &str,
//         timeout_ms: Option<u64>,
//     ) -> anyhow::Result<DateTime<Utc>> {
//         let address = Pubkey::from_str(address).map_err(|e| anyhow!(e.to_string()))?;
//         let signatures = Rpc::call(
//             move |client| {
//                 Box::pin(async move {
//                     client.get_signatures_for_address(&address).await.map_err(|e| e.into())
//                 })
//             },
//             timeout_ms,
//             rpc_mode.clone(),
//         ).await?;
//         let last_signature = signatures.last().ok_or_else(|| anyhow!("No signatures found"))?;
//         let block_time = last_signature.block_time.ok_or_else(|| anyhow::anyhow!("No block time found"))?;
//         let date_time = Utc.timestamp_opt(block_time, 0).single().ok_or_else(|| anyhow!("Invalid timestamp: {}", block_time))?;
        
//         Ok(date_time)
//     }

//     pub async fn simulate_transaction(
//         rpc_mode: RpcMode,
//         transaction: &Transaction,
//         timeout_ms: Option<u64>,
//     ) -> anyhow::Result<(RpcResponseContext, RpcSimulateTransactionResult)> {
//         let transaction_clone = transaction.clone();
    
//         // Call `Rpc::call` implementation
//         let response = Rpc::call(
//             move |client| {
//                 let transaction = transaction_clone.clone();
//                 Box::pin(async move {
//                     // Ensure `simulate_transaction` matches the expected return type
//                     client.simulate_transaction(&transaction).await.map_err(|e| e.into())
//                 })
//             },
//             timeout_ms,
//             rpc_mode,
//         )
//         .await?;
    
//         // **Explicitly check for a simulation error**
//         if let Some(err) = &response.value.err {
//             red!("\n\nSimulation Failed: {:?}\n\n", err);
//             return Err(anyhow::anyhow!("Transaction simulation failed: {:?}", err));
//         }
    
//         Ok((response.context, response.value))
//     }
    

//     pub async fn get_statuses(rpc_mode: RpcMode, signature: Signature, timeout_ms: Option<u64>) -> anyhow::Result<Vec<Option<TransactionStatus>>> {
//         let statuses_response = Rpc::call(
//             move |client| {
//                 let signature = signature.clone();
//                 Box::pin(async move {
//                     client.get_signature_statuses(&[signature]).await.map_err(|e| e.into())
//                 })
//             },
//             timeout_ms,
//             rpc_mode.clone(),
//         ).await?;

//         let statuses = statuses_response.value;

//         Ok(statuses)
        
//     }
    

//     // pub async fn estimate_transaction_fees(
//     //     rpc_mode: RpcMode,
//     //     transaction: &Transaction,
//     //     timeout_ms: Option<u64>,
//     // ) -> anyhow::Result<f64> {
//     //     // Call the simulation function
//     //     let (_context, simulation_result) = Self::simulate_transaction(rpc_mode, transaction, timeout_ms).await?;
    
//     //     // Base fee: 5,000 lamports per signature
//     //     let base_fee_lamports = transaction.signatures.len() as u64 * 5_000;
    
//     //     // Extract units consumed from the simulation
//     //     let compute_units_consumed = simulation_result.units_consumed.unwrap_or(0);
    
//     //     // Extract the compute unit price from the transaction (if set)
//     //     let compute_unit_price_micro_lamports = transaction
//     //         .message
//     //         .instructions
//     //         .iter()
//     //         .find_map(|instruction| {
//     //             // Check for ComputeBudgetInstruction::SetComputeUnitPrice
//     //             if let Some(price) = instruction.data.get(1..) {
//     //                 Some(u64::from_le_bytes(price.try_into().ok()?)) // Decode micro-lamports
//     //             } else {
//     //                 None
//     //             }
//     //         })
//     //         .unwrap_or(0);
    
//     //     // Calculate prioritization fees
//     //     let prioritization_fee_lamports = compute_units_consumed * compute_unit_price_micro_lamports / 1_000_000;
    
//     //     // Total fees in lamports
//     //     let total_fees_lamports = base_fee_lamports + prioritization_fee_lamports;
    
//     //     // Convert lamports to SOL
//     //     let total_fees_sol = total_fees_lamports as f64 / 1_000_000_000.0;
    
//     //     println!(
//     //         "Simulation result: Compute Units Consumed: {}, Base Fee (SOL): {:.9}, Prioritization Fee (SOL): {:.9}, Total Fee (SOL): {:.9}",
//     //         compute_units_consumed,
//     //         base_fee_lamports as f64 / 1_000_000_000.0,
//     //         prioritization_fee_lamports as f64 / 1_000_000_000.0,
//     //         total_fees_sol
//     //     );
    
//     //     Ok(total_fees_sol)
//     // }
    
    
// }

    
