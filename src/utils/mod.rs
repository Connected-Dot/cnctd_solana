use std::str::FromStr;

use solana_sdk::{instruction::{CompiledInstruction, Instruction}, pubkey::Pubkey, transaction::Transaction};

pub trait UuidFormatting {
    /// Removes hyphens from a UUID string to make it compatible with Solana's 32-byte seed limit
    fn to_solana_seed_format(&self) -> String;
    
    /// Restores hyphens to a UUID string that has had them removed
    fn from_solana_seed_format(&self) -> String;
    
    /// Validates if the string is a properly formatted UUID (with or without hyphens)
    fn is_valid_uuid(&self) -> bool;
}

impl UuidFormatting for str {
    fn to_solana_seed_format(&self) -> String {
        self.replace('-', "")
    }
    
    fn from_solana_seed_format(&self) -> String {
        if self.len() != 32 || self.contains('-') {
            return self.to_string(); // Return as is if not a valid hyphen-less UUID
        }
        
        format!(
            "{}-{}-{}-{}-{}",
            &self[0..8],
            &self[8..12],
            &self[12..16],
            &self[16..20],
            &self[20..32]
        )
    }
    
    fn is_valid_uuid(&self) -> bool {
        let s = self.replace('-', "");
        
        // Check if it's 32 chars after removing hyphens
        if s.len() != 32 {
            return false;
        }
        
        // Check if it only contains valid hex characters
        s.chars().all(|c| c.is_ascii_hexdigit())
    }
}

impl UuidFormatting for String {
    fn to_solana_seed_format(&self) -> String {
        self.as_str().to_solana_seed_format()
    }
    
    fn from_solana_seed_format(&self) -> String {
        self.as_str().from_solana_seed_format()
    }
    
    fn is_valid_uuid(&self) -> bool {
        self.as_str().is_valid_uuid()
    }
}


pub fn add_memo_instruction(tx: &mut Transaction, message: &str, payer_pubkey: Pubkey) {
    let memo_program_id = Pubkey::from_str("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr").unwrap();
    let memo_instruction = Instruction::new_with_bytes(
        memo_program_id,
        message.as_bytes(), // The message to display in the wallet
        vec![solana_sdk::instruction::AccountMeta::new_readonly(payer_pubkey, true)],
    );
    let compiled_instruction = CompiledInstruction {
        program_id_index: 0, // Index of the memo program in the message's account list
        accounts: vec![],
        data: memo_instruction.data,
    };
    tx.message.instructions.push(compiled_instruction);
}
