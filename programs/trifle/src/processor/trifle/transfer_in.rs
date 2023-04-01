use borsh::{BorshDeserialize, BorshSerialize};
use mpl_token_metadata::{
    state::{EscrowAuthority, Metadata, TokenMetadataAccount, ESCROW_POSTFIX, PREFIX},
    utils::{assert_derivation, assert_owned_by, is_print_edition},
};
use mpl_utils::assert_signer;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_memory::sol_memcpy,
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_token::state::{Account, Mint};

use crate::{
    error::TrifleError,
    instruction::TransferInArgs,
    state::{
        escrow_constraints::{EscrowConstraintModel, EscrowConstraintType, RoyaltyInstruction},
        transfer_effects::TransferEffects,
        trifle::Trifle,
        SolanaAccount, TRIFLE_SEED,
    },
    util::{assert_holder, pay_royalties, resize_or_reallocate_account_raw},
};

pub fn transfer_in(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: TransferInArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let trifle_info = next_account_info(account_info_iter)?;
    let trifle_authority_info = next_account_info(account_info_iter)?;
    let payer_info = next_account_info(account_info_iter)?;
    let constraint_model_info = next_account_info(account_info_iter)?;
    let escrow_info = next_account_info(account_info_iter)?;
    let escrow_mint_info = next_account_info(account_info_iter)?;
    let escrow_token_info = next_account_info(account_info_iter)?;
    let escrow_edition_info = next_account_info(account_info_iter)?;
    let attribute_mint_info = next_account_info(account_info_iter)?;
    let attribute_src_token_info = next_account_info(account_info_iter)?;
    let attribute_dst_token_info = next_account_info(account_info_iter)?;
    let attribute_metadata_info = next_account_info(account_info_iter)?;
    let attribute_edition_info = next_account_info(account_info_iter)?;
    let attribute_collection_metadata_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let _associated_token_account_program_info = next_account_info(account_info_iter)?;
    let token_metadata_program_info = next_account_info(account_info_iter)?;

    assert_signer(payer_info)?;
    assert_owned_by(attribute_metadata_info, token_metadata_program_info.key)?;

    let escrow_token_account_data = Account::unpack(&escrow_token_info.data.borrow())?;

    let attribute_metadata: Metadata = Metadata::from_account_info(attribute_metadata_info)?;
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

    // Deserialize the token accounts and perform checks.
    let attribute_src = Account::unpack(&attribute_src_token_info.data.borrow())?;
    assert!(attribute_src.mint == *attribute_mint_info.key);
    assert!(attribute_src.delegate.is_none());
    assert!(attribute_src.amount >= args.amount);

    // TODO: perform assertions on attribute_dst if it exists.
    // let attribute_dst =
    //     spl_token::state::Account::unpack(&attribute_dst_token_account.data.borrow())?;
    // msg!("past second unpack");
    // assert!(attribute_dst.mint == *attribute_mint.key);
    // assert!(attribute_dst.delegate.is_none());
    // msg!("past explicit assertions.");

    let trifle_seeds = &[
        TRIFLE_SEED.as_bytes(),
        escrow_mint_info.key.as_ref(),
        trifle_authority_info.key.as_ref(),
    ];

    let trifle_bump_seed = assert_derivation(program_id, trifle_info, trifle_seeds)?;
    let trifle_signer_seeds = &[
        TRIFLE_SEED.as_bytes(),
        escrow_mint_info.key.as_ref(),
        trifle_authority_info.key.as_ref(),
        &[trifle_bump_seed],
    ];

    let mut constraint_model =
        EscrowConstraintModel::try_from_slice(&constraint_model_info.data.borrow())
            .map_err(|_| TrifleError::InvalidEscrowConstraintModel)?;

    let constraint = constraint_model
        .constraints
        .get(&args.slot)
        .ok_or(TrifleError::InvalidEscrowConstraint)?;

    if let EscrowConstraintType::Collection(_) = constraint.constraint_type {
        let collection_key = attribute_metadata
            .collection
            .clone()
            .ok_or(TrifleError::InvalidCollection)?
            .key;

        constraint_model.validate(&collection_key, &args.slot)?;
    } else if let EscrowConstraintType::FirstCreator(_) = constraint.constraint_type {
        let asset_data = attribute_metadata.clone().into_asset_data();
        let creators_option = asset_data.creators;
        let creators = match creators_option {
            Some(x) => x,
            None => return Err(TrifleError::InvalidFirstCreator.into()),
        };
        let first_creator_option = creators.into_iter().next();
        let first_creator = match first_creator_option {
            Some(x) => x,
            None => return Err(TrifleError::InvalidFirstCreator.into()),
        };
        if !first_creator.verified {
            return Err(TrifleError::InvalidAccount.into());
        }
        constraint_model.validate(&first_creator.address, &args.slot)?;
    } else {
        constraint_model.validate(attribute_mint_info.key, &args.slot)?;
    }

    let transfer_effects = TransferEffects::from(constraint.transfer_effects);

    // Only the parent NFT holder can transfer in unless the auth_transfer_in transfer effect is enabled.
    let is_holder = assert_holder(&escrow_token_account_data, payer_info).is_ok();

    if !is_holder && transfer_effects.auth_transfer_in() {
        assert_holder(&escrow_token_account_data, trifle_authority_info)?;
    } else if !is_holder && !transfer_effects.auth_transfer_in() {
        return Err(TrifleError::MustBeHolder.into());
    }

    // check fuse options
    if transfer_effects.burn() && transfer_effects.freeze() {
        msg!("Transfer effects cannot be both burn and freeze");
        return Err(TrifleError::TransferEffectConflict.into());
    }

    // If burn is not set, create an ATA for the incoming token and perform the transfer.
    if !transfer_effects.burn() {
        // Only try to create the ATA if the account doesn't already exist.
        if *attribute_dst_token_info.owner != spl_token::ID
            && attribute_dst_token_info.lamports() == 0
        {
            // Allocate the escrow accounts new ATA.
            let create_escrow_ata_ix =
                spl_associated_token_account::instruction::create_associated_token_account(
                    payer_info.key,
                    escrow_info.key,
                    attribute_mint_info.key,
                    &spl_token::ID,
                );

            invoke(
                &create_escrow_ata_ix,
                &[
                    payer_info.clone(),
                    attribute_dst_token_info.clone(),
                    escrow_info.clone(),
                    attribute_mint_info.clone(),
                    system_program_info.clone(),
                    token_program_info.clone(),
                ],
            )?;

            // Transfer the token from the current owner into the escrow.
            let transfer_ix = spl_token::instruction::transfer(
                &spl_token::id(),
                attribute_src_token_info.key,
                attribute_dst_token_info.key,
                payer_info.key,
                &[payer_info.key],
                args.amount,
            )?;

            invoke(
                &transfer_ix,
                &[
                    attribute_src_token_info.clone(),
                    attribute_dst_token_info.clone(),
                    payer_info.clone(),
                    token_program_info.clone(),
                ],
            )?;
        }
    } else {
        let attribute_mint = Mint::unpack(&attribute_mint_info.data.borrow())?;
        if is_print_edition(
            attribute_edition_info,
            attribute_mint.decimals,
            attribute_mint.supply,
        ) {
            return Err(TrifleError::CannotBurnPrintEdition.into());
        }

        let maybe_collection_metadata_pubkey = if attribute_metadata.collection.is_some() {
            Metadata::from_account_info(attribute_collection_metadata_info)
                .map_err(|_| TrifleError::InvalidCollectionMetadata)?;

            Some(*attribute_collection_metadata_info.key)
        } else {
            None
        };

        // Burn the token from the current owner.
        let burn_ix = mpl_token_metadata::instruction::burn_nft(
            mpl_token_metadata::id(),
            *attribute_metadata_info.key,
            *payer_info.key,
            *attribute_mint_info.key,
            *attribute_src_token_info.key,
            *attribute_edition_info.key,
            *token_program_info.key,
            maybe_collection_metadata_pubkey,
        );

        let mut accounts = vec![
            attribute_metadata_info.clone(),
            payer_info.clone(),
            attribute_mint_info.clone(),
            attribute_src_token_info.clone(),
            attribute_edition_info.clone(),
            token_program_info.clone(),
        ];

        if maybe_collection_metadata_pubkey.is_some() {
            accounts.push(attribute_collection_metadata_info.clone());
        }

        // invoke_signed(&burn_ix, &accounts, &[trifle_signer_seeds])?;
        invoke(&burn_ix, &accounts)?;
    }

    if transfer_effects.freeze_parent() {
        // make sure the freeze authority is set
        let escrow_mint = Mint::unpack(&escrow_mint_info.data.borrow())?;

        if escrow_mint.freeze_authority.is_none() {
            msg!("Freeze authority is not set");
            return Err(TrifleError::FreezeAuthorityNotSet.into());
        }

        let freeze_ix = mpl_token_metadata::instruction::freeze_delegated_account(
            mpl_token_metadata::id(),
            *trifle_info.key,
            *escrow_token_info.key,
            *escrow_edition_info.key,
            *escrow_mint_info.key,
        );

        let accounts = &[
            trifle_info.clone(),
            escrow_token_info.clone(),
            escrow_edition_info.clone(),
            escrow_mint_info.clone(),
            token_program_info.clone(),
        ];

        invoke_signed(&freeze_ix, accounts, &[trifle_signer_seeds])?;
    }

    if transfer_effects.track() {
        let mut trifle = Trifle::from_account_info(trifle_info)?;

        trifle.try_add(constraint, args.slot, *attribute_mint_info.key, args.amount)?;

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
    }

    // collect and track royalties
    pay_royalties(
        RoyaltyInstruction::TransferIn,
        &mut constraint_model,
        payer_info,
        constraint_model_info,
        system_program_info,
    )?;

    // save constraint model
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

    Ok(())
}
