use borsh::BorshSerialize;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    sysvar,
};

use crate::instruction::TrifleInstruction;

#[allow(clippy::too_many_arguments)]
pub fn create_trifle_account(
    program_id: &Pubkey,
    escrow: &Pubkey,
    metadata: &Pubkey,
    mint: &Pubkey,
    token_account: &Pubkey,
    edition: &Pubkey,
    trifle_account: &Pubkey,
    trifle_authority: &Pubkey,
    escrow_constraint_model: &Pubkey,
    payer: &Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*escrow, false),
        AccountMeta::new(*metadata, false),
        AccountMeta::new_readonly(*mint, false),
        AccountMeta::new_readonly(*token_account, false),
        AccountMeta::new_readonly(*edition, false),
        AccountMeta::new(*trifle_account, false),
        AccountMeta::new_readonly(*trifle_authority, true),
        AccountMeta::new(*escrow_constraint_model, false),
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(mpl_token_metadata::id(), false),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        AccountMeta::new_readonly(sysvar::instructions::id(), false),
    ];

    Instruction {
        program_id: *program_id,
        accounts,
        data: TrifleInstruction::CreateTrifleAccount.try_to_vec().unwrap(),
    }
}
