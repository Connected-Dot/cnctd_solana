use std::str::FromStr;

use base64::Engine;
use sha2::{Digest, Sha256};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction}, message::Message, pubkey::Pubkey, signature::{Keypair, Signer}, signer::EncodableKey, transaction::Transaction
};
use borsh::{BorshSerialize, BorshDeserialize};
use anyhow::{Result, anyhow};

use crate::utils::add_memo_instruction;

pub struct CnctdSolana {
    rpc_url: String,
}

impl CnctdSolana {
    pub fn new(rpc_url: &str) -> Result<Self> {
        Ok(Self {
            rpc_url: rpc_url.to_string(),
        })
    }

    pub async fn send_instruction<T: BorshSerialize>(
        &self,
        program_id: &str,
        instruction_name: &str,  // Instruction name must match your Anchor function
        instruction_data: T,
        account_pubkeys: Vec<String>,
    ) -> Result<String> {
        let program_id = Pubkey::from_str(program_id)
            .map_err(|_| anyhow!("Invalid program ID"))?;
    
        // ✅ Prepend the discriminator
        let mut data = get_discriminator(instruction_name).to_vec();
        data.extend(borsh::to_vec(&instruction_data)?);
    
        println!("data (with discriminator): {:?}", data);
    
        let client = RpcClient::new(self.rpc_url.clone());
    
        let keypair_path = "/Users/kyleebner/.config/solana/id.json";
        let payer = Keypair::read_from_file(keypair_path)
            .map_err(|_| anyhow!("Unable to read keypair file"))?;
    
        // ✅ Convert String pubkeys into AccountMeta
        let mut accounts = account_pubkeys
            .into_iter()
            .map(|pubkey_str| {
                Pubkey::from_str(&pubkey_str)
                    .map(|pubkey| AccountMeta::new(pubkey, false))
                    .map_err(|_| anyhow!("Invalid public key: {}", pubkey_str))
            })
            .collect::<Result<Vec<_>>>()?;

        accounts.push(AccountMeta::new(payer.pubkey(), true));
    
        let instruction = Instruction::new_with_bytes(program_id, &data, accounts);
    
        let recent_blockhash = client.get_latest_blockhash()?;
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );
    
        let signature = client.send_and_confirm_transaction(&transaction)?;
        Ok(signature.to_string())
    }

    pub async fn get_transaction<T: BorshSerialize>(
        &self,
        program_id: &str,
        instruction_name: &str,  // Instruction name must match your Anchor function
        instruction_data: T,
        account_pubkeys: Vec<String>,
    ) -> Result<String> {
        let program_id = Pubkey::from_str(program_id)
            .map_err(|_| anyhow!("Invalid program ID"))?;
    
        // ✅ Prepend the discriminator
        let mut data = get_discriminator(instruction_name).to_vec();
        data.extend(borsh::to_vec(&instruction_data)?);
    
        println!("data (with discriminator): {:?}", data);
    
        let client = RpcClient::new(self.rpc_url.clone());
    
        let keypair_path = "/Users/kyleebner/.config/solana/id.json";
        let payer = Keypair::read_from_file(keypair_path)
            .map_err(|_| anyhow!("Unable to read keypair file"))?;
    
        // ✅ Convert String pubkeys into AccountMeta
        let mut accounts = account_pubkeys
            .into_iter()
            .map(|pubkey_str| {
                Pubkey::from_str(&pubkey_str)
                    .map(|pubkey| AccountMeta::new(pubkey, false))
                    .map_err(|_| anyhow!("Invalid public key: {}", pubkey_str))
            })
            .collect::<Result<Vec<_>>>()?;

        accounts.push(AccountMeta::new(payer.pubkey(), true));
    
        let instruction = Instruction::new_with_bytes(program_id, &data, accounts);
    
        let recent_blockhash = client.get_latest_blockhash()?;
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );
        
    
        let signature = client.send_and_confirm_transaction(&transaction)?;
        Ok(signature.to_string())
    }

    pub async fn create_unsigned_transaction<T: BorshSerialize>(
        &self,
        program_id: &str,
        instruction_name: &str,
        instruction_data: T,
        account_pubkeys: Vec<String>,
        payer_address: String,
        memo: Option<String>
    ) -> Result<String> {
        let program_id = Pubkey::from_str(program_id)
            .map_err(|_| anyhow!("Invalid program ID"))?;
    
        let mut data = get_discriminator(instruction_name).to_vec();
        data.extend(borsh::to_vec(&instruction_data)?);
    
        let accounts = account_pubkeys
            .into_iter()
            .map(|pubkey_str| {
                Pubkey::from_str(&pubkey_str)
                    .map(|pubkey| AccountMeta::new(pubkey, false))
                    .map_err(|_| anyhow!("Invalid public key: {}", pubkey_str))
            })
            .collect::<Result<Vec<_>>>()?;
    
        let instruction = Instruction::new_with_bytes(program_id, &data, accounts);

        let payer_address = Pubkey::from_str(&payer_address)
            .map_err(|_| anyhow!("Invalid payer public key"))?;    
        
        let message = Message::new(&[instruction], Some(&payer_address));
        let transaction = Transaction::new_unsigned(message);
        let transaction = if let Some(memo) = memo {
            let mut tx = transaction;
            add_memo_instruction(&mut tx, &memo, payer_address);
            tx
        } else {
            transaction
        };
    
        let serialized_tx = bincode::serialize(&transaction)?;
        let base64_tx = base64::engine::general_purpose::STANDARD.encode(serialized_tx);
    
        Ok(base64_tx)
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