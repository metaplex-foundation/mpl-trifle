use borsh::{BorshDeserialize, BorshSerialize};
use mpl_token_metadata::{
    state::{EscrowAuthority, Metadata, TokenMetadataAccount, ESCROW_POSTFIX, PREFIX},
    utils::{assert_derivation, assert_owned_by},
};
use mpl_utils::assert_signer;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_memory::sol_memcpy,
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_token::state::Account;

use crate::{
    error::TrifleError,
    instruction::TransferOutArgs,
    state::{
        escrow_constraints::{EscrowConstraintModel, RoyaltyInstruction},
        transfer_effects::TransferEffects,
        trifle::Trifle,
        SolanaAccount, TRIFLE_SEED,
    },
    util::{assert_holder, pay_royalties, resize_or_reallocate_account_raw},
};

pub fn transfer_out(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: TransferOutArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let trifle_info = next_account_info(account_info_iter)?;
    let constraint_model_info = next_account_info(account_info_iter)?;
    let escrow_info = next_account_info(account_info_iter)?;
    let escrow_token_info = next_account_info(account_info_iter)?;
    let escrow_mint_info = next_account_info(account_info_iter)?;
    let escrow_metadata_info = next_account_info(account_info_iter)?;
    let escrow_edition_info = next_account_info(account_info_iter)?;
    let payer_info = next_account_info(account_info_iter)?;
    let trifle_authority_info = next_account_info(account_info_iter)?;
    let attribute_mint_info = next_account_info(account_info_iter)?;
    let attribute_src_token_info = next_account_info(account_info_iter)?;
    let attribute_dst_token_info = next_account_info(account_info_iter)?;
    let attribute_metadata_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;
    let _ata_program_info = next_account_info(account_info_iter)?;
    let _spl_token_program_info = next_account_info(account_info_iter)?;
    let token_metadata_program_info = next_account_info(account_info_iter)?;
    let sysvar_ix_account_info = next_account_info(account_info_iter)?;

    assert_owned_by(attribute_metadata_info, &mpl_token_metadata::id())?;
    let _attribute_metadata: Metadata = Metadata::from_account_info(attribute_metadata_info)?;

    let mut escrow_seeds = vec![
        PREFIX.as_bytes(),
        token_metadata_program_info.key.as_ref(),
        escrow_mint_info.key.as_ref(),
    ];

    let escrow_auth = EscrowAuthority::Creator(*trifle_info.key);
    for seed in escrow_auth.to_seeds() {
        escrow_seeds.push(seed);
    }

    escrow_seeds.push(ESCROW_POSTFIX.as_bytes());
    assert_derivation(token_metadata_program_info.key, escrow_info, &escrow_seeds)?;

    let trifle_seeds = &[
        TRIFLE_SEED.as_bytes(),
        escrow_mint_info.key.as_ref(),
        trifle_authority_info.key.as_ref(),
    ];

    let trifle_bump_seed = assert_derivation(program_id, trifle_info, trifle_seeds)?;

    // Derive the seeds for PDA signing.
    let trifle_signer_seeds = &[
        TRIFLE_SEED.as_bytes(),
        escrow_mint_info.key.as_ref(),
        trifle_authority_info.key.as_ref(),
        &[trifle_bump_seed],
    ];

    assert_signer(payer_info)?;
    // assert_signer(trifle_authority_info)?;

    let escrow_token_account_data = Account::unpack(&escrow_token_info.data.borrow())?;

    // Transfer the token out of the escrow
    let transfer_ix = mpl_token_metadata::escrow::transfer_out_of_escrow(
        *token_metadata_program_info.key,
        *escrow_info.key,
        *escrow_metadata_info.key,
        *payer_info.key,
        *attribute_mint_info.key,
        *attribute_src_token_info.key,
        *attribute_dst_token_info.key,
        *escrow_mint_info.key,
        *escrow_token_info.key,
        Some(*trifle_info.key),
        args.amount,
    );

    invoke_signed(
        &transfer_ix,
        &[
            escrow_info.clone(),
            payer_info.clone(),
            attribute_mint_info.clone(),
            attribute_src_token_info.clone(),
            attribute_dst_token_info.clone(),
            attribute_metadata_info.clone(),
            escrow_mint_info.clone(),
            escrow_token_info.clone(),
            trifle_info.clone(),
            escrow_metadata_info.clone(),
            sysvar_ix_account_info.clone(),
        ],
        &[trifle_signer_seeds],
    )?;

    // Update the Trifle account
    let mut trifle = Trifle::from_account_info(trifle_info)?;
    trifle.try_remove(args.slot.clone(), *attribute_mint_info.key, args.amount)?;

    let mut constraint_model =
        EscrowConstraintModel::try_from_slice(&constraint_model_info.data.borrow())
            .map_err(|_| TrifleError::InvalidEscrowConstraintModel)?;

    let constraint = constraint_model
        .constraints
        .get(&args.slot)
        .ok_or(TrifleError::InvalidEscrowConstraint)?;

    let transfer_effects = TransferEffects::from(constraint.transfer_effects);

    // Only the parent NFT holder can transfer out unless the auth_transfer_out transfer effect is enabled.
    let is_holder = assert_holder(&escrow_token_account_data, payer_info).is_ok();

    if !is_holder && transfer_effects.auth_transfer_out() {
        assert_holder(&escrow_token_account_data, trifle_authority_info)?;
    } else if !is_holder && !transfer_effects.auth_transfer_out() {
        return Err(TrifleError::MustBeHolder.into());
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

    sol_memcpy(
        &mut constraint_model_info
            .try_borrow_mut_data()
            .map_err(|_| TrifleError::FailedToBorrowAccountData)?,
        &serialized_data,
        serialized_data.len(),
    );

    let serialized_data = trifle
        .try_to_vec()
        .map_err(|_| TrifleError::FailedToSerialize)?;

    resize_or_reallocate_account_raw(
        trifle_info,
        payer_info,
        system_program_info,
        serialized_data.len(),
    )?;

    sol_memcpy(
        &mut trifle_info
            .try_borrow_mut_data()
            .map_err(|_| TrifleError::FailedToBorrowAccountData)?,
        &serialized_data,
        serialized_data.len(),
    );

    if trifle.is_empty() && transfer_effects.freeze_parent() {
        let escrow_token = Account::unpack(&escrow_token_info.data.borrow())?;
        if escrow_token.is_frozen() {
            msg!("Last token transferred out of escrow. Unfreezing the escrow token account.");

            let thaw_ix = mpl_token_metadata::instruction::thaw_delegated_account(
                mpl_token_metadata::id(),
                *trifle_info.key,
                *escrow_token_info.key,
                *escrow_edition_info.key,
                *escrow_mint_info.key,
            );

            invoke_signed(
                &thaw_ix,
                &[
                    trifle_info.to_owned(),
                    escrow_token_info.to_owned(),
                    escrow_edition_info.to_owned(),
                    escrow_mint_info.to_owned(),
                    _spl_token_program_info.to_owned(),
                ],
                &[trifle_signer_seeds],
            )?;
        }
    }

    Ok(())
}
