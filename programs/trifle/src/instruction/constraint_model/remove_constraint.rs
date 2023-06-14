use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use crate::instruction::TrifleInstruction;

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct RemoveConstraintFromEscrowConstraintModelArgs {
    pub constraint_name: String,
}

pub fn remove_constraint_from_escrow_constraint_model(
    program_id: Pubkey,
    escrow_constraint_model: Pubkey,
    payer: Pubkey,
    update_authority: Pubkey,
    constraint_name: String,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(escrow_constraint_model, false),
        AccountMeta::new(payer, true),
        AccountMeta::new(update_authority, true),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
    ];

    Instruction {
        program_id,
        accounts,
        data: TrifleInstruction::RemoveConstraintFromEscrowConstraintModel(
            RemoveConstraintFromEscrowConstraintModelArgs { constraint_name },
        )
        .try_to_vec()
        .unwrap(),
    }
}
