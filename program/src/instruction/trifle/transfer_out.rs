use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    sysvar,
};

use crate::{instruction::TrifleInstruction, util::account_meta_new_or_readonly};

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct TransferOutArgs {
    pub slot: String,
    pub amount: u64,
}

#[allow(clippy::too_many_arguments)]
pub fn transfer_out(
    program_id: Pubkey,
    trifle_account: Pubkey,
    constraint_model: Pubkey,
    escrow_account: Pubkey,
    escrow_token_account: Pubkey,
    escrow_mint: Pubkey,
    escrow_metadata: Pubkey,
    escrow_edition: Option<Pubkey>,
    payer: Pubkey,
    trifle_authority: Pubkey,
    attribute_mint: Pubkey,
    attribute_src_token_account: Pubkey,
    attribute_dst_token_account: Pubkey,
    attribute_metadata: Pubkey,
    slot: String,
    amount: u64,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(trifle_account, false),
        AccountMeta::new(constraint_model, false),
        AccountMeta::new_readonly(escrow_account, false),
        AccountMeta::new(escrow_token_account, false),
        AccountMeta::new(escrow_mint, false),
        AccountMeta::new(escrow_metadata, false),
        account_meta_new_or_readonly(escrow_edition, program_id),
        AccountMeta::new(payer, true),
        AccountMeta::new_readonly(trifle_authority, false),
        AccountMeta::new_readonly(attribute_mint, false),
        AccountMeta::new(attribute_src_token_account, false),
        AccountMeta::new(attribute_dst_token_account, false),
        AccountMeta::new_readonly(attribute_metadata, false),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        AccountMeta::new_readonly(spl_associated_token_account::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(mpl_token_metadata::id(), false),
        AccountMeta::new_readonly(sysvar::instructions::id(), false),
    ];

    let data = TrifleInstruction::TransferOut(TransferOutArgs { slot, amount })
        .try_to_vec()
        .unwrap();

    Instruction {
        program_id,
        accounts,
        data,
    }
}
