use borsh::{BorshDeserialize, BorshSerialize};
use mpl_token_metadata::error::MetadataError;
use mpl_utils::{
    assert_derivation, assert_owned_by, assert_signer, create_or_allocate_account_raw,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_memory::sol_memcpy,
    pubkey::Pubkey,
};

use crate::{
    error::TrifleError,
    state::{
        escrow_constraints::{EscrowConstraintModel, RoyaltyInstruction},
        trifle::Trifle,
        Key, TRIFLE_SEED,
    },
    util::pay_royalties,
};

pub fn create_trifle_account(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let escrow_info = next_account_info(account_info_iter)?;
    let metadata_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let token_account_info = next_account_info(account_info_iter)?;
    let edition_info = next_account_info(account_info_iter)?;
    let trifle_info = next_account_info(account_info_iter)?;
    let trifle_authority_info = next_account_info(account_info_iter)?;
    let escrow_constraint_model_info = next_account_info(account_info_iter)?;
    let payer_info = next_account_info(account_info_iter)?;
    let _tm_program_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;
    let sysvar_ix_account_info = next_account_info(account_info_iter)?;

    let trifle_pda_bump = assert_derivation(
        program_id,
        trifle_info,
        &[
            TRIFLE_SEED.as_bytes(),
            mint_info.key.as_ref(),
            trifle_authority_info.key.as_ref(),
        ],
        TrifleError::DerivedKeyInvalid,
    )?;

    assert_signer(payer_info)?;
    assert_signer(trifle_authority_info)?;
    assert_owned_by(
        escrow_info,
        system_program_info.key,
        TrifleError::IncorrectOwner,
    )?;
    if !escrow_info.data_is_empty() {
        return Err(MetadataError::AlreadyInitialized.into());
    }
    assert_owned_by(
        escrow_constraint_model_info,
        program_id,
        TrifleError::IncorrectOwner,
    )?;
    assert_owned_by(
        metadata_info,
        &mpl_token_metadata::ID,
        TrifleError::IncorrectOwner,
    )?;
    assert_owned_by(mint_info, &spl_token::id(), TrifleError::IncorrectOwner)?;
    assert_owned_by(
        token_account_info,
        &spl_token::id(),
        TrifleError::IncorrectOwner,
    )?;

    let escrow_constraint_model_key =
        Key::try_from_slice(&escrow_constraint_model_info.data.borrow()[0..1])?;

    if escrow_constraint_model_key != Key::EscrowConstraintModel {
        return Err(TrifleError::InvalidEscrowConstraintModel.into());
    }

    let trifle_signer_seeds = &[
        TRIFLE_SEED.as_bytes(),
        mint_info.key.as_ref(),
        trifle_authority_info.key.as_ref(),
        &[trifle_pda_bump],
    ];

    let trifle = Trifle {
        token_escrow: escrow_info.key.to_owned(),
        escrow_constraint_model: escrow_constraint_model_info.key.to_owned(),
        ..Default::default()
    };

    let mut constraint_model =
        EscrowConstraintModel::try_from_slice(&escrow_constraint_model_info.data.borrow())
            .map_err(|_| TrifleError::InvalidEscrowConstraintModel)?;
    pay_royalties(
        RoyaltyInstruction::CreateTrifle,
        &mut constraint_model,
        payer_info,
        escrow_constraint_model_info,
        system_program_info,
    )?;

    let serialized_data = constraint_model
        .try_to_vec()
        .map_err(|_| TrifleError::FailedToSerialize)?;

    sol_memcpy(
        &mut escrow_constraint_model_info
            .try_borrow_mut_data()
            .map_err(|_| TrifleError::FailedToBorrowAccountData)?,
        &serialized_data,
        serialized_data.len(),
    );

    let serialized_data = trifle
        .try_to_vec()
        .map_err(|_| TrifleError::FailedToSerialize)?;

    create_or_allocate_account_raw(
        *program_id,
        trifle_info,
        system_program_info,
        payer_info,
        serialized_data.len(),
        trifle_signer_seeds,
    )?;

    sol_memcpy(
        &mut trifle_info
            .try_borrow_mut_data()
            .map_err(|_| TrifleError::FailedToBorrowAccountData)?,
        &serialized_data,
        serialized_data.len(),
    );

    let create_escrow_account_ix = mpl_token_metadata::escrow::create_escrow_account(
        mpl_token_metadata::ID,
        *escrow_info.key,
        *metadata_info.key,
        *mint_info.key,
        *token_account_info.key,
        *edition_info.key,
        *payer_info.key,
        Some(*trifle_info.key),
    );

    let account_infos = vec![
        escrow_info.clone(),
        metadata_info.clone(),
        mint_info.clone(),
        token_account_info.clone(),
        edition_info.clone(),
        payer_info.clone(),
        system_program_info.clone(),
        trifle_info.clone(),
        sysvar_ix_account_info.clone(),
    ];

    msg!("Creating token escrow.");
    invoke_signed(
        &create_escrow_account_ix,
        &account_infos,
        &[trifle_signer_seeds],
    )?;

    Ok(())
}
