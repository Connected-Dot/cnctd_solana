use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

pub trait BorshSize {
    // Return the size in bytes when serialized with Borsh
    fn borsh_size() -> usize;
}

// Implement for fixed-size types
impl BorshSize for Pubkey {
    fn borsh_size() -> usize { 32 }
}

impl BorshSize for u8 {
    fn borsh_size() -> usize { 1 }
}

impl BorshSize for u64 {
    fn borsh_size() -> usize { 8 }
}

// Custom fixed-length string type
#[derive(Debug, Serialize, Deserialize, Clone, BorshSerialize, BorshDeserialize)]
pub struct FixedString32(pub String);

impl BorshSize for FixedString32 {
    fn borsh_size() -> usize { 
        // 4 bytes for length prefix + 32 bytes for content
        4 + 32 
    }
}

impl ToString for FixedString32 {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}


// Enhance the FilterableAccount trait
pub trait FilterableAccount: BorshDeserialize {
    fn discriminator() -> [u8; 8];
    
    // New method for calculating offsets dynamically based on field order
    fn get_field_offset_by_index(_field_index: usize) -> Option<usize> {
        // Each implementation can define its field order and sizes
        // to automatically calculate offsets
        None
    }
    
    fn get_field_offset(field_name: &str) -> Option<usize>;
    fn serialize_field_value(field_name: &str, value: &serde_json::Value) -> Option<Vec<u8>>;
}