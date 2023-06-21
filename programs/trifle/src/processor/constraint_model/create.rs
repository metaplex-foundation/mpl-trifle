use borsh::BorshSerialize;
use mpl_utils::{assert_derivation, create_or_allocate_account_raw};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_memory::sol_memcpy,
    pubkey::Pubkey,
};

use crate::{
    error::TrifleError,
    instruction::CreateEscrowConstraintModelAccountArgs,
    state::{
        escrow_constraints::{EscrowConstraintModel, RoyaltyInstruction},
        Key, ESCROW_SEED,
    },
    util::pay_royalties,
};

pub fn create_escrow_constraints_model_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: CreateEscrowConstraintModelAccountArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let escrow_constraint_model_info = next_account_info(account_info_iter)?;
    let payer_info = next_account_info(account_info_iter)?;
    let update_authority_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;

    let mut escrow_constraint_model = EscrowConstraintModel {
        key: Key::EscrowConstraintModel,
        name: args.name.to_owned(),
        creator: payer_info.key.to_owned(),
        update_authority: update_authority_info.key.to_owned(),
        schema_uri: args.schema_uri.to_owned(),
        ..Default::default()
    };

    let bump = assert_derivation(
        program_id,
        escrow_constraint_model_info,
        &[
            ESCROW_SEED.as_bytes(),
            payer_info.key.as_ref(),
            args.name.as_bytes(),
        ],
        TrifleError::DerivedKeyInvalid,
    )?;

    let escrow_constraint_model_seeds = &[
        ESCROW_SEED.as_ref(),
        payer_info.key.as_ref(),
        args.name.as_ref(),
        &[bump],
    ];

    pay_royalties(
        RoyaltyInstruction::CreateModel,
        &mut escrow_constraint_model,
        payer_info,
        escrow_constraint_model_info,
        system_program_info,
    )?;

    let serialized_data = escrow_constraint_model
        .try_to_vec()
        .map_err(|_| TrifleError::FailedToSerialize)?;

    create_or_allocate_account_raw(
        *program_id,
        escrow_constraint_model_info,
        system_program_info,
        payer_info,
        serialized_data.len(),
        escrow_constraint_model_seeds,
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
