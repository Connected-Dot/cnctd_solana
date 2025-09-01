use std::str::FromStr;
use solana_sdk::{
    instruction::{AccountMeta, Instruction}, message::compiled_instruction::CompiledInstruction, pubkey::Pubkey, transaction::Transaction
};
pub use filterable_account::FilterableAccount;
pub mod filterable_account;

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
