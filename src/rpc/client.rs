use std::str::FromStr;

use base64::Engine;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcSimulateTransactionConfig};
use solana_sdk::{
    account::Account, commitment_config::CommitmentConfig, hash::hashv, instruction::{AccountMeta, Instruction}, message::Message, pubkey::Pubkey, signature::{Keypair, Signature, Signer}, signer::EncodableKey, transaction::Transaction
};
use borsh::{BorshSerialize, BorshDeserialize};
use anyhow::{Result, anyhow};
use spl_token_2022::solana_program;

pub trait TransactionExt {
    fn to_base64_string(&self) -> Result<String>;
}

impl TransactionExt for Transaction {
    fn to_base64_string(&self) -> Result<String> {
        let serialized_tx = bincode::serialize(self)?;
        let base64_tx = base64::engine::general_purpose::STANDARD.encode(serialized_tx);
    
        Ok(base64_tx)
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PdaState {
    Exists { address: Pubkey },  // Use Pubkey instead of String
    NeedsReinitialization { address: Pubkey }, // Use Pubkey instead of String
    NotFound,
}

pub struct CnctdSolana {
    pub rpc_url: String,
    pub signer_keypair: Option<Keypair>,
    pub client: RpcClient,
}

impl CnctdSolana {
    pub fn new(rpc_url: &str) -> Result<Self> {
        Ok(Self {
            rpc_url: rpc_url.to_string(),
            signer_keypair: None,
            client: RpcClient::new(rpc_url.to_string()),
        })
    }

    pub fn get_initialize_discriminator() -> Vec<u8> {
        let sighash = hashv(&[b"global:initialize"]); // Mimicking Anchor-style sighash
        sighash.as_ref()[..8].to_vec() // First 8 bytes as the discriminator
    }

    pub async fn get_latest_blockhash(&self) -> Result<String> {
        let client = &self.client;
        let blockhash = client.get_latest_blockhash().await?;

        Ok(blockhash.to_string())
    }

    pub async fn get_minimum_balance_for_rent_exemption(&self, data_len: usize) -> Result<u64> {
        let client = &self.client;
        let min_balance = client.get_minimum_balance_for_rent_exemption(data_len).await?;

        Ok(min_balance)
    }

    // pub async fn send_instruction<T: BorshSerialize>(
    //     &self,
    //     program_id: &str,
    //     instruction_name: &str,  // Instruction name must match your Anchor function
    //     instruction_data: T,
    //     account_pubkeys: Vec<String>,
    // ) -> Result<String> {
    //     let program_id = Pubkey::from_str(program_id)
    //         .map_err(|_| anyhow!("Invalid program ID"))?;
    
    //     // ✅ Prepend the discriminator
    //     let mut data = get_discriminator(instruction_name).to_vec();
    //     data.extend(borsh::to_vec(&instruction_data)?);
    
    //     println!("data (with discriminator): {:?}", data);
    
    //     let client = &self.client;
    
    //     let keypair_path = "/Users/kyleebner/.config/solana/id.json";
    //     let payer = Keypair::read_from_file(keypair_path)
    //         .map_err(|_| anyhow!("Unable to read keypair file"))?;
    
    //     // ✅ Convert String pubkeys into AccountMeta
    //     let mut accounts = account_pubkeys
    //         .into_iter()
    //         .map(|pubkey_str| {
    //             Pubkey::from_str(&pubkey_str)
    //                 .map(|pubkey| AccountMeta::new(pubkey, false))
    //                 .map_err(|_| anyhow!("Invalid public key: {}", pubkey_str))
    //         })
    //         .collect::<Result<Vec<_>>>()?;

    //     accounts.push(AccountMeta::new(payer.pubkey(), true));
    
    //     let instruction = Instruction::new_with_bytes(program_id, &data, accounts);
    
    //     let recent_blockhash = client.get_latest_blockhash()?;
    //     let transaction = Transaction::new_signed_with_payer(
    //         &[instruction],
    //         Some(&payer.pubkey()),
    //         &[&payer],
    //         recent_blockhash,
    //     );
    
    //     let signature = client.send_and_confirm_transaction(&transaction)?;
    //     Ok(signature.to_string())
    // }

    // pub async fn get_transaction<T: BorshSerialize>(
    //     &self,
    //     program_id: &str,
    //     instruction_name: &str,  // Instruction name must match your Anchor function
    //     instruction_data: T,
    //     account_pubkeys: Vec<String>,
    // ) -> Result<String> {
    //     let program_id = Pubkey::from_str(program_id)
    //         .map_err(|_| anyhow!("Invalid program ID"))?;
    
    //     // ✅ Prepend the discriminator
    //     let mut data = get_discriminator(instruction_name).to_vec();
    //     data.extend(borsh::to_vec(&instruction_data)?);
    
    //     println!("data (with discriminator): {:?}", data);
    
    //     let client = &self.client;
    
    //     let keypair_path = "/Users/kyleebner/.config/solana/id.json";
    //     let payer = Keypair::read_from_file(keypair_path)
    //         .map_err(|_| anyhow!("Unable to read keypair file"))?;
    
    //     // ✅ Convert String pubkeys into AccountMeta
    //     let mut accounts = account_pubkeys
    //         .into_iter()
    //         .map(|pubkey_str| {
    //             Pubkey::from_str(&pubkey_str)
    //                 .map(|pubkey| AccountMeta::new(pubkey, false))
    //                 .map_err(|_| anyhow!("Invalid public key: {}", pubkey_str))
    //         })
    //         .collect::<Result<Vec<_>>>()?;

    //     accounts.push(AccountMeta::new(payer.pubkey(), true));
    
    //     let instruction = Instruction::new_with_bytes(program_id, &data, accounts);
    
    //     let recent_blockhash = client.get_latest_blockhash()?;
    //     let transaction = Transaction::new_signed_with_payer(
    //         &[instruction],
    //         Some(&payer.pubkey()),
    //         &[&payer],
    //         recent_blockhash,
    //     );
        
    
    //     let signature = client.send_and_confirm_transaction(&transaction)?;
    //     Ok(signature.to_string())
    // }


    pub async fn create_unsigned_transaction<T: BorshSerialize>(
        &self,
        program_id: Pubkey,
        instruction_name: &str,
        instruction_data: T,
        accounts: Vec<AccountMeta>, // Now contains payer info
    ) -> Result<Transaction> {
        let mut data = Self::get_discriminator(instruction_name).to_vec();
        data.extend(borsh::to_vec(&instruction_data)?);
    
        // Find the first writable signer (payer)
        let payer_pubkey = accounts
            .iter()
            .find(|meta| meta.is_signer && meta.is_writable)
            .map(|meta| meta.pubkey)
            .ok_or_else(|| anyhow!("No writable signer found to act as payer"))?;
    
        let instruction = Instruction::new_with_bytes(program_id, &data, accounts);
    
        let message = Message::new(&[instruction], Some(&payer_pubkey));
        let transaction = Transaction::new_unsigned(message);

        Ok(transaction)
    }

    pub async fn create_instruction(
        &self,
        program_id: Pubkey,
        instruction_name: &str,
        instruction_data: impl BorshSerialize,
        accounts: Vec<AccountMeta>,
    ) -> Result<Instruction> {
        let mut data = Self::get_discriminator(instruction_name).to_vec();
        data.extend(borsh::to_vec(&instruction_data)?);
    
        let instruction = Instruction::new_with_bytes(program_id, &data, accounts);
    
        Ok(instruction)
    }

    pub async fn get_pda_pubkey(&self, program_pubkey: Pubkey, seed: &str) -> Result<Pubkey> {
        let rpc_client = RpcClient::new(self.rpc_url.clone());
    
        let (pda, _bump) = Pubkey::find_program_address(&[seed.as_bytes()], &program_pubkey);
    
        match rpc_client.get_account(&pda).await {
            Ok(account) => {
                if account.lamports > 0 {
                    Ok(pda)
                } else {
                    Err(anyhow!("PDA account has no lamports"))
                }
            }
            Err(e) => Err(anyhow!("Error fetching PDA account: {}", e)),
        }
    }

    pub async fn get_account(&self, pubkey: Pubkey) -> Result<Account> {
        let rpc_client = RpcClient::new(self.rpc_url.clone());
    
        match rpc_client.get_account(&pubkey).await {
            Ok(account) => Ok(account),
            Err(e) => Err(anyhow!("Error fetching account: {}", e)),
        }
    }
    
    pub async fn get_account_data<T: BorshDeserialize>(&self, pubkey: Pubkey) -> Result<T> {
        let rpc_client = RpcClient::new(self.rpc_url.clone());
        
        match rpc_client.get_account(&pubkey).await {
            Ok(account) => {
                // Check if account data has at least the discriminator (8 bytes)
                if account.data.len() < 8 {
                    return Err(anyhow!("Account data is too short to be a valid Anchor account"));
                }
                
                // Skip the 8-byte discriminator and use the rest of the data
                let data_slice = &account.data[8..];
                
                // Try to deserialize from the entire remaining data
                let data = T::deserialize(&mut &data_slice[..])
                    .map_err(|e| anyhow!("Failed to deserialize account data: {}", e))?;
                    
                Ok(data)
            },
            Err(e) => Err(anyhow!("Error fetching account: {}", e)),
        }
    }
    
    pub async fn sign_and_confirm_transaction(
        &self,
        transaction: &Transaction,
        additional_signers: Option<&[&Keypair]>,
    ) -> Result<Signature> {
        let client = &self.client;
        
        // Get a recent blockhash
        let recent_blockhash = client.get_latest_blockhash().await?;
        
        // Clone transaction and set the blockhash
        let mut signed_transaction = transaction.clone();
        signed_transaction.message.recent_blockhash = recent_blockhash;
        
        // Get the primary signer
        let signer_keypair = self.signer_keypair.as_ref()
            .ok_or_else(|| anyhow!("Signer keypair is not set"))?;
        
        // Sign with just the primary signer or with additional signers
        if let Some(additional) = additional_signers {
            let mut all_signers = vec![signer_keypair];
            all_signers.extend(additional.iter());
            
            let all_signers_refs: Vec<&Keypair> = all_signers.iter().map(|k| *k).collect();
            signed_transaction.sign(&all_signers_refs, recent_blockhash);
        } else {
            signed_transaction.sign(&[signer_keypair], recent_blockhash);
        }
        
        // Send and confirm the signed transaction
        let signature = client.send_and_confirm_transaction(&signed_transaction).await?;
        
        Ok(signature)
    }
    
    pub async fn simulate_transaction(
        &self,
        transaction: &Transaction,
        additional_signers: Option<&[&Keypair]>,
    ) -> Result<solana_client::rpc_response::RpcSimulateTransactionResult> {
        let client = &self.client;
        let recent_blockhash = client.get_latest_blockhash().await?;
        
        // Clone the transaction and update blockhash
        let mut signed_transaction = transaction.clone();
        signed_transaction.message.recent_blockhash = recent_blockhash;
        
        // Sign the transaction for simulation
        if let Some(signer) = &self.signer_keypair {
            if let Some(additional) = additional_signers {
                let mut all_signers = vec![signer];
                all_signers.extend(additional);
                
                signed_transaction.sign(&all_signers, recent_blockhash);
            } else {
                signed_transaction.sign(&[signer], recent_blockhash);
            }
        } else {
            return Err(anyhow!("No signer keypair provided for transaction simulation"));
        }
        
        // Use simulation config to handle PDA creation scenarios
        let config = RpcSimulateTransactionConfig {
            sig_verify: false,
            replace_recent_blockhash: true,
            commitment: Some(CommitmentConfig::confirmed()),
            accounts: None,
            encoding: None,
            min_context_slot: None,
            inner_instructions: true,
        };
        
        let simulation_result = client.simulate_transaction_with_config(
            &signed_transaction, 
            config
        ).await?;
        
        Ok(simulation_result.value)
    }
    
    pub async fn estimate_transaction_fee(
        &self,
        transaction: &Transaction,
        additional_signers: Option<&[&Keypair]>,
    ) -> anyhow::Result<u64> {
        // Simulate the transaction to get compute units
        let simulation = self.simulate_transaction(transaction, additional_signers).await?;
        
        // Base fee: 5,000 lamports per signature
        let base_fee = transaction.signatures.len() as u64 * 5_000;
        
        // Extract units consumed from the simulation
        let compute_units_consumed = simulation.units_consumed.unwrap_or(0);
        
        // Compute unit fee calculation
        let compute_unit_price = 5; // micro-lamports per compute unit (5 / 10^6)
        let compute_fee = (compute_units_consumed as u128 * compute_unit_price as u128 / 1_000_000) as u64;
        
        // Total fee
        let total_fee = base_fee + compute_fee;
        
        println!("Estimated fee: {} lamports (base: {}, compute: {}, units: {})",
                 total_fee, base_fee, compute_fee, compute_units_consumed);
        
        Ok(total_fee)
    }

    pub fn get_discriminator(instruction_name: &str) -> [u8; 8] {
        let mut hasher = Sha256::new();
        hasher.update(format!("global:{}", instruction_name).as_bytes());
        let hash = hasher.finalize();
        let mut discriminator = [0u8; 8];
        discriminator.copy_from_slice(&hash[..8]);
        discriminator
    }
    
    pub async fn wait_for_confirmation(&self, signature: &Signature, max_retries: Option<u32>) -> anyhow::Result<bool> {
        let max_retries = max_retries.unwrap_or(60); // Default to 30 retries
        let rpc_client = &self.client;
        let mut attempts = 0;
        let sleep_time = std::time::Duration::from_millis(500); // 500ms between checks
        
        while attempts < max_retries {
            match rpc_client.confirm_transaction(signature).await {
                Ok(_) => {
                    return Ok(true); // Transaction confirmed successfully
                },
                Err(e) => {
                    // Check if the error indicates the transaction is still in flight
                    if e.to_string().contains("not found") || e.to_string().contains("still in flight") {
                        // Transaction not yet processed, continue waiting
                        attempts += 1;
                        tokio::time::sleep(sleep_time).await;
                    } else {
                        // Other RPC error, likely a failure
                        return Err(anyhow::anyhow!("Transaction confirmation failed: {}", e));
                    }
                }
            }
        }
        
        // If we reach here, we've exceeded max_retries
        Err(anyhow::anyhow!("Transaction confirmation timed out after {} attempts", max_retries))
    }
    
}

