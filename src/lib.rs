pub use solana_sdk::*;
pub use solana_client::*;
pub use solana_account_decoder_client_types::*;
pub use solana_address::*;
pub use solana_compute_budget_interface::*;
pub use solana_system_interface::instruction as system_instruction;
pub use solana_cluster_type::ClusterType;
// pub use spl_token_2022::ID as SPL_TOKEN_PROGRAM_ID_2022;
// pub use spl_token::ID as SPL_TOKEN_PROGRAM_ID;
// pub use spl_associated_token_account::ID as SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID;
// pub use spl_associated_token_account::solana_program::system_program as SYSTEM_PROGRAM_ID;
// pub use sysvar::rent::ID as SYSVAR_RENT_ID;
// pub use mpl_token_metadata::ID as MPL_TOKEN_METADATA_PROGRAM_ID;
// pub use spl_associated_token_account_interface::program::ID as ASSOCIATED_TOKEN_PROGRAM_ID;
pub use spl_associated_token_account::{get_associated_token_address_with_program_id, get_associated_token_address};
pub use crate::utils::get_associated_token_address_with_program_id_address;
/// SPL Token (legacy)
pub const SPL_TOKEN_PROGRAM_ID: Address =
    Address::from_str_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

/// SPL Token-2022
pub const SPL_TOKEN_PROGRAM_ID_2022: Address =
    Address::from_str_const("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");

/// Associated Token Account program (classic crate)
pub const SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID: Address =
    Address::from_str_const("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

/// Associated Token Account program (interface crate) — same address
pub const ASSOCIATED_TOKEN_PROGRAM_ID: Address =
    Address::from_str_const("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

/// System Program
pub const SYSTEM_PROGRAM_ID: Address =
    Address::from_str_const("11111111111111111111111111111111");

/// Sysvar: Rent
pub const SYSVAR_RENT_ID: Address =
    Address::from_str_const("SysvarRent111111111111111111111111111111111");

/// Metaplex Token Metadata
pub const MPL_TOKEN_METADATA_PROGRAM_ID: Address =
    Address::from_str_const("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

pub const METAPLEX_METADATA_ADDRESS: &str = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";

// pub mod payments;
pub mod rpc;
pub mod utils;

