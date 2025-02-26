use std::str::FromStr;

use base64::Engine;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    account::Account, hash::hashv, instruction::{AccountMeta, Instruction}, message::Message, pubkey::Pubkey, signature::{Keypair, Signer}, signer::EncodableKey, transaction::Transaction
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
    rpc_url: String,
}

impl CnctdSolana {
    pub fn new(rpc_url: &str) -> Result<Self> {
        Ok(Self {
            rpc_url: rpc_url.to_string(),
        })
    }

    pub fn get_initialize_discriminator() -> Vec<u8> {
        let sighash = hashv(&[b"global:initialize"]); // Mimicking Anchor-style sighash
        sighash.as_ref()[..8].to_vec() // First 8 bytes as the discriminator
    }

    pub async fn get_latest_blockhash(&self) -> Result<String> {
        let client = RpcClient::new(self.rpc_url.clone());
        let blockhash = client.get_latest_blockhash().await?;

        Ok(blockhash.to_string())
    }

    pub async fn get_minimum_balance_for_rent_exemption(&self, data_len: usize) -> Result<u64> {
        let client = RpcClient::new(self.rpc_url.clone());
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
    
    //     let client = RpcClient::new(self.rpc_url.clone());
    
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
    
    //     let client = RpcClient::new(self.rpc_url.clone());
    
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
        let mut data = get_discriminator(instruction_name).to_vec();
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
                if account.data.len() < 8 {
                    return Err(anyhow!("Account data is too short to be a valid Anchor account"));
                }
    
                // Limit the slice to the allocated space
                let allocated_space = 74; // Adjust this based on actual allocation
                let mut data_slice = &account.data[8..(8 + allocated_space).min(account.data.len())];
    
                let data = T::deserialize_reader(&mut data_slice)
                    .map_err(|e| anyhow!("Failed to deserialize account data: {}", e))?;
    
                Ok(data)
            },
            Err(e) => Err(anyhow!("Error fetching account: {}", e)),
        }
    }
    
    
    
}

fn get_discriminator(instruction_name: &str) -> [u8; 8] {
    let mut hasher = Sha256::new();
    hasher.update(format!("global:{}", instruction_name).as_bytes());
    let hash = hasher.finalize();
    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&hash[..8]);
    discriminator
}