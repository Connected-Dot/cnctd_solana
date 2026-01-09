# CLAUDE.md - cnctd_solana

> Reference for the Solana blockchain integration library.

## Purpose

Solana blockchain integration providing token operations, account management, and transaction building utilities for the cnctd ecosystem.

## Key Exports

```rust
// Re-exports from Solana SDK
pub use solana_sdk::{pubkey::Pubkey, signature::Keypair, transaction::Transaction, ...};
pub use solana_client::rpc_client::RpcClient;

// Program ID constants
pub const SPL_TOKEN_PROGRAM_ID: Pubkey;
pub const SPL_TOKEN_PROGRAM_ID_2022: Pubkey;
pub const ASSOCIATED_TOKEN_PROGRAM_ID: Pubkey;
pub const SYSTEM_PROGRAM_ID: Pubkey;
pub const SYSVAR_RENT_ID: Pubkey;
pub const MPL_TOKEN_METADATA_PROGRAM_ID: Pubkey;

// Helper functions
pub fn get_associated_token_address_with_program_id_address(...) -> Pubkey;

pub mod rpc;    // RPC operations
pub mod utils;  // Utility functions
```

## Usage Example

```rust
use cnctd_solana::{Pubkey, Keypair, rpc::CnctdSolana};

let client = CnctdSolana::new("https://api.mainnet-beta.solana.com");
let balance = client.get_balance(&pubkey)?;
```

## Ecosystem Role

- **Used by**: cnctd.world (NFT releases, payments)
- **Related**: cnctd_solana_programs (on-chain programs)

## Version

**0.1.11**

---

*Part of the cnctd monorepo. See `../../../CLAUDE.md` for ecosystem context.*
