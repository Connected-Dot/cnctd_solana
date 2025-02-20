use std::str::FromStr;

use solana_sdk::{instruction::{CompiledInstruction, Instruction}, pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction};

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
