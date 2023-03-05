use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    sysvar,
};

use crate::instruction::TrifleInstruction;

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct AddTokensConstraintToEscrowConstraintModelArgs {
    pub constraint_name: String,
    pub tokens: Vec<Pubkey>,
    pub token_limit: u64,
    pub transfer_effects: u16,
}

#[allow(clippy::too_many_arguments)]
pub fn add_tokens_constraint_to_escrow_constraint_model(
    program_id: &Pubkey,
    escrow_constraint_model: &Pubkey,
    payer: &Pubkey,
    update_authority: &Pubkey,
    constraint_name: String,
    token_limit: u64,
    tokens: Vec<Pubkey>,
    transfer_effects: u16,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*escrow_constraint_model, false),
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(*update_authority, true),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        AccountMeta::new_readonly(sysvar::instructions::id(), false),
    ];

    Instruction {
        program_id: *program_id,
        accounts,
        data: TrifleInstruction::AddTokensConstraintToEscrowConstraintModel(
            AddTokensConstraintToEscrowConstraintModelArgs {
                constraint_name,
                tokens,
                token_limit,
                transfer_effects,
            },
        )
        .try_to_vec()
        .unwrap(),
    }
}
