use std::str::FromStr;
use solana_sdk::{
    instruction::{AccountMeta, Instruction}, message::compiled_instruction::CompiledInstruction, pubkey::Pubkey, transaction::Transaction
};
pub use solana_address::Address;
pub use filterable_account::{FilterableAccount, BorshSize, FixedString32};
pub use uuid_formatting::UuidFormatting;

use crate::ASSOCIATED_TOKEN_PROGRAM_ID;

pub mod filterable_account;
pub mod uuid_formatting;

pub fn add_memo_instruction(tx: &mut Transaction, message: &str, payer_pubkey: Pubkey) {
    let memo_program_id =
        Pubkey::from_str("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr").unwrap();

    // High-level instruction for the memo program
    let memo_ix = Instruction::new_with_bytes(
        memo_program_id,
        message.as_bytes(),
        vec![AccountMeta::new_readonly(payer_pubkey, true)],
    );

    // Ensure the message’s account list contains the program and the payer,
    // then compute the correct indices for the compiled instruction
    let keys = &mut tx.message.account_keys;

    let prog_idx = match keys.iter().position(|k| *k == memo_ix.program_id) {
        Some(i) => i as u8,
        None => {
            keys.push(memo_ix.program_id);
            (keys.len() - 1) as u8
        }
    };

    let mut acct_indices: Vec<u8> = Vec::with_capacity(memo_ix.accounts.len());
    for meta in &memo_ix.accounts {
        let idx = match keys.iter().position(|k| *k == meta.pubkey) {
            Some(i) => i as u8,
            None => {
                keys.push(meta.pubkey);
                (keys.len() - 1) as u8
            }
        };
        acct_indices.push(idx);
    }

    let compiled = CompiledInstruction::new_from_raw_parts(
        prog_idx,
        acct_indices,
        memo_ix.data.clone(),
    );
    tx.message.instructions.push(compiled);
}

pub fn get_associated_token_address_with_program_id_address(
    wallet_address: &Address,
    token_mint_address: &Address,
    token_program_id: &Address,
) -> Address {
    let seeds: &[&[u8]] = &[
        wallet_address.as_ref(),
        token_program_id.as_ref(),
        token_mint_address.as_ref(),
    ];
    let associated_token_program_id = ASSOCIATED_TOKEN_PROGRAM_ID;
    let (ata, _bump) = Address::find_program_address(seeds, &associated_token_program_id);
    ata
}
