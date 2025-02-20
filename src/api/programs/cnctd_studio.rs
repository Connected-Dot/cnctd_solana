use std::str::FromStr;

use bincode::serialize;
use serde::{Deserialize, Serialize};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{client::Client, commitment_config::CommitmentConfig, instruction::{AccountMeta, Instruction}, message::Message, pubkey::Pubkey, signature::{read_keypair_file, Keypair}, signer::Signer, system_program, transaction::Transaction};
use anyhow::anyhow;

#[derive(Serialize, Deserialize, Clone)]
pub struct Album {
    pub album_id: Pubkey,    // Unique album identifier
    pub creator: Pubkey,     // Wallet that created the album (original owner)
    pub price: u64,          // Price in USDC (amount in lamports)
    pub credits: Vec<CreditSplit>, // Artist payout splits
    pub resale_allowed: bool, // Whether resale is allowed
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreditSplit {
    pub artist_wallet: Pubkey,  // Artist’s wallet address
    pub percentage: u8,         // Percentage share (0-100)
}

#[derive(Serialize, Deserialize)]
struct MintAlbum {
    album: Album,
}


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
        let pubkey = payer.pubkey();
    
        let album = Album {
            album_id: Keypair::new().pubkey(),
            creator: pubkey,
            price: 100_000_000,
            credits: vec![
                CreditSplit {
                    artist_wallet: Keypair::new().pubkey(),
                    percentage: 50,
                },
                CreditSplit {
                    artist_wallet: Keypair::new().pubkey(),
                    percentage: 50,
                },
            ],
            resale_allowed: true,
        };
    
        let instruction_data = serialize(&MintAlbum { album })?;
        let accounts = vec![AccountMeta::new_readonly(pubkey, false)];
        let instruction = Instruction::new_with_bincode(self.program_id, &instruction_data, accounts);
    
        let recent_blockhash = client.get_latest_blockhash().await?;
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );
    
        println!("Sending transaction... {:?}", transaction);
        let signature = client.send_and_confirm_transaction(&transaction).await?;
        Ok(signature.to_string())

    }

    // fn test_test() {
    //     let program_id = "AsE3BweZsNJa2oT6sbvNh1UXmLLJmcfYY1hvGJXL9T8L";
    //     let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    //     let payer = read_keypair_file(&anchor_wallet).unwrap();
    
    //     let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());
    //     let program_id = Pubkey::from_str(program_id).unwrap();
    //     let program = client.program(program_id).unwrap();
    
    //     let tx = program
    //         .request()
    //         .accounts(cnctd_solana_programs::accounts::Initialize {})
    //         .args(cnctd_solana_programs::instruction::Test {})
    //         .send()
    //         .expect("");
    
    //     println!("Your transaction signature {}", tx);
    // }
}