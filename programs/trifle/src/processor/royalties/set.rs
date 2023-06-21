use borsh::{BorshDeserialize, BorshSerialize};
use mpl_utils::assert_derivation;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_memory::sol_memcpy,
    pubkey::Pubkey,
};

use crate::{
    error::TrifleError,
    instruction::SetRoyaltiesArgs,
    state::{
        escrow_constraints::{EscrowConstraintModel, RoyaltyInstruction},
        ESCROW_SEED,
    },
    util::{pay_royalties, resize_or_reallocate_account_raw},
};

pub fn set_royalties(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: SetRoyaltiesArgs,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let constraint_model_info = next_account_info(accounts_iter)?;
    let payer_info = next_account_info(accounts_iter)?;
    let _update_authority_info = next_account_info(accounts_iter)?;
    let system_program_info = next_account_info(accounts_iter)?;
    let _sysvar_instruction_info = next_account_info(accounts_iter)?;

    let bump = assert_derivation(
        program_id,
        constraint_model_info,
        &[
            ESCROW_SEED.as_bytes(),
            payer_info.key.as_ref(),
            args.name.as_bytes(),
        ],
        TrifleError::DerivedKeyInvalid,
    )?;

    let _constraint_model_seeds = &[
        ESCROW_SEED.as_ref(),
        payer_info.key.as_ref(),
        args.name.as_ref(),
        &[bump],
    ];

    let mut constraint_model =
        EscrowConstraintModel::try_from_slice(&constraint_model_info.data.borrow())
            .map_err(|_| TrifleError::InvalidEscrowConstraintModel)?;

    // Royalties are set on a per-instruction basis, so loop through each
    // IX:Royalty pair and set the royalty in the map.
    for ix_type in args.royalties {
        constraint_model
            .royalties
            .entry(ix_type.0)
            .or_insert_with(|| ix_type.1);
    }

    // collect fees and save the model.
    pay_royalties(
        RoyaltyInstruction::TransferOut,
        &mut constraint_model,
        payer_info,
        constraint_model_info,
        system_program_info,
    )?;

    let serialized_data = constraint_model
        .try_to_vec()
        .map_err(|_| TrifleError::FailedToSerialize)?;

    resize_or_reallocate_account_raw(
        constraint_model_info,
        payer_info,
        system_program_info,
        serialized_data.len(),
    )?;

    sol_memcpy(
        &mut constraint_model_info
            .try_borrow_mut_data()
            .map_err(|_| TrifleError::FailedToBorrowAccountData)?,
        &serialized_data,
        serialized_data.len(),
    );

    Ok(())
}
