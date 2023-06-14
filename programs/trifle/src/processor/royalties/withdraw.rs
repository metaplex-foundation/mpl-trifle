use borsh::{BorshDeserialize, BorshSerialize};
use mpl_token_metadata::{assertions::assert_keys_equal, utils::assert_derivation};
use mpl_utils::assert_signer;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke_signed,
    program_memory::sol_memcpy,
    pubkey,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

use crate::{
    error::TrifleError,
    instruction::WithdrawRoyaltiesArgs,
    state::{escrow_constraints::EscrowConstraintModel, ESCROW_SEED},
    util::resize_or_reallocate_account_raw,
};

pub fn withdraw_royalties(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: WithdrawRoyaltiesArgs,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let constraint_model_info = next_account_info(accounts_iter)?;
    let payer_info = next_account_info(accounts_iter)?;
    let update_authority_info = next_account_info(accounts_iter)?;
    let destination_info = next_account_info(accounts_iter)?;
    let new_dest_info = next_account_info(accounts_iter)?;
    let system_program_info = next_account_info(accounts_iter)?;
    let _sysvar_instruction_info = next_account_info(accounts_iter)?;

    assert_signer(payer_info)?;
    assert_keys_equal(
        new_dest_info.key,
        &pubkey!("2Hm4qr8xLwQWoBErjQWp4sTND4p2FqyDmppxQyrkTV99"),
    )?;

    let bump = assert_derivation(
        program_id,
        constraint_model_info,
        &[
            ESCROW_SEED.as_bytes(),
            update_authority_info.key.as_ref(),
            args.name.as_bytes(),
        ],
    )?;

    let constraint_model_seeds = &[
        ESCROW_SEED.as_ref(),
        update_authority_info.key.as_ref(),
        args.name.as_ref(),
        &[bump],
    ];

    let mut constraint_model =
        EscrowConstraintModel::try_from_slice(&constraint_model_info.data.borrow())
            .map_err(|_| TrifleError::InvalidEscrowConstraintModel)?;

    // Check that the payer is the update authority before paying out royalties.
    if payer_info.key == update_authority_info.key {
        // Transfer the creator royalties balance to the destination account
        // and set the balance to 0 afterwards.
        invoke_signed(
            &system_instruction::transfer(
                constraint_model_info.key,
                destination_info.key,
                constraint_model.royalty_balance,
            ),
            &[
                constraint_model_info.clone(),
                destination_info.clone(),
                constraint_model_info.clone(),
                system_program_info.clone(),
            ],
            &[constraint_model_seeds],
        )?;

        constraint_model.royalty_balance = 0;
    }

    let serialized_data = constraint_model
        .try_to_vec()
        .map_err(|_| TrifleError::FailedToSerialize)?;

    // Transfer the remaining balance to the Metaplex DAO. The untracked balance
    // (account.lamports - rent - royalty_balance) is the total collected protocol fees.
    invoke_signed(
        &system_instruction::transfer(
            constraint_model_info.key,
            new_dest_info.key,
            constraint_model_info
                .lamports()
                .checked_sub(constraint_model.royalty_balance)
                .ok_or(TrifleError::NumericalOverflow)?
                .checked_sub(Rent::get()?.minimum_balance(serialized_data.len()))
                .ok_or(TrifleError::NumericalOverflow)?,
        ),
        &[
            constraint_model_info.clone(),
            new_dest_info.clone(),
            constraint_model_info.clone(),
            system_program_info.clone(),
        ],
        &[constraint_model_seeds],
    )?;

    if payer_info.key == update_authority_info.key {
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
    }

    Ok(())
}
