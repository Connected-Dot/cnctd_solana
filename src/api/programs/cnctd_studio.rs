use std::str::FromStr;

use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{instruction::{AccountMeta, Instruction}, message::Message, pubkey::Pubkey, signature::Keypair, signer::Signer, system_program, transaction::Transaction};
use anyhow::anyhow;

pub struct CnctdStudioProgram {
    pub program_id: Pubkey,
    pub rpc_url: String,
}

impl CnctdStudioProgram {
    pub fn new(program_id: &str, rpc_url: &str) -> anyhow::Result<Self> {
        let pubkey = Pubkey::from_str(program_id)?;

        Ok(Self {
            program_id: pubkey,
            rpc_url: rpc_url.to_string(),
        })
    }

    pub async fn test(&self) -> anyhow::Result<String> {
        let client = RpcClient::new(self.rpc_url.clone());
        let keypair_path = "/Users/kyleebner/.config/solana/id.json";

        let payer = solana_sdk::signature::read_keypair_file(keypair_path).map_err(|_| anyhow!("Unable to read keypair file"))?;
        let program_pubkey = self.program_id;

        let instruction = Instruction::new_with_bincode::<()> (
            program_pubkey,
            &(), // No extra input
            vec![
                AccountMeta::new(payer.pubkey(), true),  // The signer (payer)
                AccountMeta::new_readonly(system_program::ID, false), // System program
            ],
        );

        println!("Instruction: {:?}", instruction);
    
        let blockhash = client.get_latest_blockhash().await?;
        let message = Message::new(&[instruction], Some(&payer.pubkey()));
        let transaction = Transaction::new(&[&payer], message, blockhash);
    
        let signature = client.send_and_confirm_transaction(&transaction).await?;
        Ok(signature.to_string())

    }
}