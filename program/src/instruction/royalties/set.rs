use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    sysvar,
};

use crate::{instruction::TrifleInstruction, state::escrow_constraints::RoyaltyInstruction};

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct SetRoyaltiesArgs {
    pub name: String,
    pub royalties: Vec<(RoyaltyInstruction, u64)>,
}

pub fn set_royalties(
    program_id: &Pubkey,
    escrow_constraint_model: &Pubkey,
    payer: &Pubkey,
    update_authority: &Pubkey,
    name: String,
    royalties: Vec<(RoyaltyInstruction, u64)>,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*escrow_constraint_model, false),
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(*update_authority, false),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        AccountMeta::new_readonly(sysvar::instructions::id(), false),
    ];

    Instruction {
        program_id: *program_id,
        accounts,
        data: TrifleInstruction::SetRoyalties(SetRoyaltiesArgs { name, royalties })
            .try_to_vec()
            .unwrap(),
    }
}
