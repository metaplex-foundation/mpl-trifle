use std::str::FromStr;

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
pub struct WithdrawRoyaltiesArgs {
    pub name: String,
}

pub fn withdraw_royalties(
    program_id: &Pubkey,
    escrow_constraint_model: &Pubkey,
    payer: &Pubkey,
    update_authority: &Pubkey,
    destination: &Pubkey,
    name: String,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*escrow_constraint_model, false),
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(*update_authority, false),
        AccountMeta::new_readonly(*destination, false),
        AccountMeta::new_readonly(
            Pubkey::from_str("2Hm4qr8xLwQWoBErjQWp4sTND4p2FqyDmppxQyrkTV99").unwrap(),
            false,
        ),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        AccountMeta::new_readonly(sysvar::instructions::id(), false),
    ];

    Instruction {
        program_id: *program_id,
        accounts,
        data: TrifleInstruction::WithdrawRoyalties(WithdrawRoyaltiesArgs { name })
            .try_to_vec()
            .unwrap(),
    }
}
