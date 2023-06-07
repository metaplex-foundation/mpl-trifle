use mpl_token_metadata::state::{Metadata, TokenMetadataAccount};

use mpl_utils::assert_owned_by;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    sysvar::instructions::{get_instruction_relative, load_current_index_checked},
};

use crate::{
    error::TrifleError,
    instruction::AddCollectionConstraintToEscrowConstraintModelArgs,
    processor::constraint_model::add_constraint_to_escrow_constraint_model,
    state::escrow_constraints::{EscrowConstraint, EscrowConstraintType},
    util::is_creation_instruction,
};

pub fn add_collection_constraint_to_escrow_constraint_model(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: AddCollectionConstraintToEscrowConstraintModelArgs,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    accounts_iter.next(); // skip the escrow constraint model
    accounts_iter.next(); // skip the payer
    accounts_iter.next(); // skip the update authority
    let collection_mint_info = next_account_info(accounts_iter)?;
    let collection_metadata_info = next_account_info(accounts_iter)?;
    accounts_iter.next(); // skip the system program
    let sysvar_instruction_info = next_account_info(accounts_iter)?;

    assert_owned_by(
        collection_mint_info,
        &spl_token::id(),
        TrifleError::IncorrectOwner,
    )?;
    assert_owned_by(
        collection_metadata_info,
        &mpl_token_metadata::id(),
        TrifleError::IncorrectOwner,
    )?;

    Metadata::from_account_info(collection_metadata_info)
        .map_err(|_| TrifleError::InvalidCollectionMetadata)?;

    let constraint = EscrowConstraint {
        constraint_type: EscrowConstraintType::Collection(*collection_mint_info.key),
        token_limit: args.token_limit,
        transfer_effects: args.transfer_effects,
    };

    // Check if the previous instruction was a creation instruction, so we don't double-charge protocol fees.
    let mut creation_ix = false;
    if load_current_index_checked(sysvar_instruction_info)? > 0 {
        let prev_ix = get_instruction_relative(-1, sysvar_instruction_info)?;
        creation_ix = is_creation_instruction(*prev_ix.data.first().unwrap_or(&255));
    }

    add_constraint_to_escrow_constraint_model(
        program_id,
        accounts,
        !creation_ix,
        args.constraint_name,
        constraint,
    )
}
