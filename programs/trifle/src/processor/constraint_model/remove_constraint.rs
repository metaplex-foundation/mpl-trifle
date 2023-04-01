use borsh::BorshSerialize;
use mpl_token_metadata::utils::assert_owned_by;
use mpl_utils::assert_signer;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_memory::sol_memcpy,
    pubkey::Pubkey,
};

use crate::{
    error::TrifleError,
    instruction::RemoveConstraintFromEscrowConstraintModelArgs,
    state::{
        escrow_constraints::{EscrowConstraintModel, RoyaltyInstruction},
        SolanaAccount,
    },
    util::{pay_royalties, resize_or_reallocate_account_raw},
};

pub fn remove_constraint_from_escrow_constraint_model(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: RemoveConstraintFromEscrowConstraintModelArgs,
) -> ProgramResult {
    let mut accounts_iter = accounts.iter();
    let escrow_constraint_model_info = next_account_info(&mut accounts_iter)?;
    let payer_info = next_account_info(&mut accounts_iter)?;
    let update_authority_info = next_account_info(&mut accounts_iter)?;
    let system_program_info = next_account_info(&mut accounts_iter)?;

    assert_signer(payer_info)?;
    assert_signer(update_authority_info)?;
    assert_owned_by(escrow_constraint_model_info, program_id)?;
    // assert update authority matches ecm update authority;
    let mut escrow_constraint_model =
        EscrowConstraintModel::from_account_info(escrow_constraint_model_info)?;

    if escrow_constraint_model.update_authority != *update_authority_info.key {
        return Err(TrifleError::InvalidUpdateAuthority.into());
    }

    // remove the constraint by key.
    escrow_constraint_model
        .constraints
        .remove(&args.constraint_name);

    pay_royalties(
        RoyaltyInstruction::RemoveConstraint,
        &mut escrow_constraint_model,
        payer_info,
        escrow_constraint_model_info,
        system_program_info,
    )?;

    let serialized_data = escrow_constraint_model
        .try_to_vec()
        .map_err(|_| TrifleError::FailedToSerialize)?;

    // resize the account to the new size.
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
