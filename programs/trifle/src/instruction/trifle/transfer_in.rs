use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

use crate::{instruction::TrifleInstruction, util::account_meta_new_or_readonly};

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct TransferInArgs {
    pub slot: String,
    pub amount: u64,
}

#[allow(clippy::too_many_arguments)]
pub fn transfer_in(
    program_id: Pubkey,
    trifle_account: Pubkey,
    trifle_authority: Pubkey,
    payer: Pubkey,
    constraint_model: Pubkey,
    escrow_account: Pubkey,
    escrow_mint: Option<Pubkey>,
    escrow_token_account: Option<Pubkey>,
    escrow_edition: Option<Pubkey>,
    attribute_mint: Pubkey,
    attribute_src_token_account: Pubkey,
    attribute_dst_token_account: Option<Pubkey>,
    attribute_metadata: Option<Pubkey>,
    attribute_edition: Option<Pubkey>,
    attribute_collection_metadata: Option<Pubkey>,
    slot: String,
    amount: u64,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(trifle_account, false),
        AccountMeta::new(trifle_authority, false),
        AccountMeta::new(payer, true),
        AccountMeta::new(constraint_model, false),
        AccountMeta::new_readonly(escrow_account, false),
        account_meta_new_or_readonly(escrow_mint, program_id),
        account_meta_new_or_readonly(escrow_token_account, program_id),
        account_meta_new_or_readonly(escrow_edition, program_id),
        AccountMeta::new(attribute_mint, false),
        AccountMeta::new(attribute_src_token_account, false),
        account_meta_new_or_readonly(attribute_dst_token_account, program_id),
        // TODO: attribute metadata doesn't need to be writable unless burning.
        account_meta_new_or_readonly(attribute_metadata, program_id),
        account_meta_new_or_readonly(attribute_edition, program_id),
        account_meta_new_or_readonly(attribute_collection_metadata, program_id),
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(spl_associated_token_account::id(), false),
        AccountMeta::new_readonly(mpl_token_metadata::id(), false),
    ];

    let data = TrifleInstruction::TransferIn(TransferInArgs { slot, amount })
        .try_to_vec()
        .unwrap();

    Instruction {
        program_id,
        accounts,
        data,
    }
}
