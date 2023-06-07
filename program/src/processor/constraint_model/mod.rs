mod add_collection_constraint;
mod add_first_creator_constraint;
mod add_none_constraint;
mod add_tokens_constraint;
mod create;
mod remove_constraint;
pub use add_collection_constraint::*;
pub use add_first_creator_constraint::*;
pub use add_none_constraint::*;
pub use add_tokens_constraint::*;
use borsh::{BorshDeserialize, BorshSerialize};
pub use create::*;
use mpl_utils::{assert_derivation, assert_owned_by, assert_signer};
pub use remove_constraint::*;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_memory::sol_memcpy,
    pubkey::Pubkey,
};

use crate::{
    error::TrifleError,
    state::{
        escrow_constraints::{EscrowConstraint, EscrowConstraintModel, RoyaltyInstruction},
        ESCROW_SEED,
    },
    util::{pay_royalties, resize_or_reallocate_account_raw},
};

fn add_constraint_to_escrow_constraint_model(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    take_fees: bool,
    constraint_name: String,
    escrow_constraint: EscrowConstraint,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let escrow_constraint_model_info = next_account_info(account_info_iter)?;
    let payer_info = next_account_info(account_info_iter)?;
    let update_authority_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;

    assert_owned_by(
        escrow_constraint_model_info,
        program_id,
        TrifleError::IncorrectOwner,
    )?;
    assert_signer(payer_info)?;
    assert_signer(update_authority_info)?;

    let mut escrow_constraint_model: EscrowConstraintModel =
        EscrowConstraintModel::try_from_slice(&escrow_constraint_model_info.data.borrow())?;

    if escrow_constraint_model.update_authority != *update_authority_info.key {
        return Err(TrifleError::InvalidUpdateAuthority.into());
    }

    assert_derivation(
        program_id,
        escrow_constraint_model_info,
        &[
            ESCROW_SEED.as_bytes(),
            payer_info.key.as_ref(),
            escrow_constraint_model.name.as_bytes(),
        ],
        TrifleError::DerivedKeyInvalid,
    )?;

    if escrow_constraint_model
        .constraints
        .contains_key(&constraint_name)
    {
        return Err(TrifleError::ConstraintAlreadyExists.into());
    }

    escrow_constraint_model
        .constraints
        .insert(constraint_name, escrow_constraint);

    // Pay royalties and protocol fees if we haven't already.
    if take_fees {
        pay_royalties(
            RoyaltyInstruction::AddConstraint,
            &mut escrow_constraint_model,
            payer_info,
            escrow_constraint_model_info,
            system_program_info,
        )?;
    }

    let serialized_data = escrow_constraint_model
        .try_to_vec()
        .map_err(|_| TrifleError::FailedToSerialize)?;

    resize_or_reallocate_account_raw(
        escrow_constraint_model_info,
        payer_info,
        system_program_info,
        serialized_data.len(),
    )?;

    sol_memcpy(
        &mut escrow_constraint_model_info
            .try_borrow_mut_data()
            .map_err(|_| TrifleError::FailedToBorrowAccountData)?,
        &serialized_data,
        serialized_data.len(),
    );

    Ok(())
}
