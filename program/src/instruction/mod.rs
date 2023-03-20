mod constraint_model;
mod royalties;
mod trifle;
pub use constraint_model::*;
pub use royalties::*;
pub use trifle::*;

use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;

#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(ShankInstruction, Debug, BorshSerialize, Clone, BorshDeserialize)]
#[rustfmt::skip]
pub enum TrifleInstruction {
    /// Create an constraint model to be used by one or many escrow accounts.
    #[account(0, writable, name = "escrow_constraint_model", desc = "Constraint model account")]
    #[account(1, writable, signer, name = "payer", desc = "Wallet paying for the transaction and new account, will be set as the creator of the constraint model")]
    #[account(2, name = "update_authority", desc = "Update authority of the constraint model")]
    #[account(3, name = "system_program", desc = "System program")]
    CreateEscrowConstraintModelAccount(CreateEscrowConstraintModelAccountArgs),


    /// Creates a Trifle Account -- used to model token inventory in a Token Escrow account.
    #[account(0, writable, name = "escrow", desc = "Escrow account")]
    #[account(1, writable, name = "metadata", desc = "Metadata account")]
    #[account(2, name = "mint", desc = "Mint account")]
    #[account(3, writable, name = "token_account", desc = "Token account (base token)")]
    #[account(4, name = "edition", desc = "Edition account")]
    #[account(5, writable, name = "trifle_account", desc = "Trifle account")]
    #[account(6, signer, name = "trifle_authority", desc = "Trifle Authority - the account that can sign transactions for the trifle account")]
    #[account(7, writable, name = "constraint_model", desc = "Escrow constraint model")]
    #[account(8, writable, signer, name = "payer", desc = "Wallet paying for the transaction")]
    #[account(9, name = "token_metadata_program", desc = "Token Metadata program")]
    #[account(10, name = "system_program", desc = "System program")]
    #[account(11, name="sysvar_instructions", desc="Instructions sysvar account")]
    CreateTrifleAccount,

    /// Transfer tokens into the Trifle escrow account.
    #[default_optional_accounts]
    #[account(0, writable, name = "trifle", desc = "The trifle account to use")]
    #[account(1, writable, name = "trifle_authority", desc = "Trifle Authority - the account that can sign transactions for the trifle account")]
    #[account(2, writable, signer, name = "payer", desc = "Wallet paying for the transaction" )]
    #[account(3, writable, name = "constraint_model", desc = "The escrow constraint model of the Trifle account")]
    #[account(4, name = "escrow", desc = "The escrow account of the Trifle account")]
    #[account(5, optional, name = "escrow_mint", desc = "The escrow account's base token mint")]
    #[account(6, optional, writable, name = "escrow_token", desc = "The token account of the escrow account's base token")]
    #[account(7, optional, writable, name = "escrow_edition", desc = "The freeze authority of the escrow account's base token mint")]
    #[account(8, optional, writable, name = "attribute_mint", desc = "The mint of the attribute token")]
    #[account(9, optional, writable, name = "attribute_src_token", desc = "The token account that the attribute token is being transferred from")]
    #[account(10, optional, writable, name = "attribute_dst_token", desc = "The token account that the attribute token is being transferred to (pda of the escrow account)")]
    #[account(11, optional, writable, name = "attribute_metadata", desc = "The metadata account of the attribute token")]
    #[account(12, optional, writable, name = "attribute_edition", desc = "The edition account of the attribute token")]
    #[account(13, optional, writable, name = "attribute_collection_metadata", desc = "The collection metadata account of the attribute token")]
    #[account(14, name = "system_program", desc = "System program")]
    #[account(15, name = "spl_token", desc = "Token program")]
    #[account(16, name = "spl_associated_token_account", desc = "Associated token account program")]
    #[account(17, name = "token_metadata_program", desc = "Token Metadata program")]
    TransferIn(TransferInArgs),

    /// Transfer tokens out of the Trifle escrow account.
    #[default_optional_accounts]
    #[account(0, writable, name="trifle_account", desc="The trifle account to use")]
    #[account(1, writable, name="constraint_model", desc="The constraint model to check against")]
    #[account(2, name="escrow_account", desc="The escrow account attached to the NFT")]
    #[account(3, writable, name="escrow_token_account", desc="The token account holding the NFT the escrow is attached to")]
    #[account(4, writable, name="escrow_mint", desc="The mint of the NFT the escrow is attached to")]
    #[account(5, writable, name="escrow_metadata", desc="The metadata account for the escrow mint")]
    #[account(6, optional, writable, name="escrow_edition", desc="The edition of the NFT the escrow is attached to")]
    #[account(7, writable, signer, name = "payer", desc = "Wallet paying for the transaction")]
    #[account(8, name = "trifle_authority", desc = "Trifle Authority - the account that can sign transactions for the trifle account")]
    #[account(9, name="attribute_mint", desc="The mint of the attribute")]
    #[account(10, writable, name="attribute_src_token_account", desc="The token account the attribute is being transferred from")]
    #[account(11, writable, name="attribute_dst_token_account", desc="The token account the attribute is being transferred to")]
    #[account(12, name="attribute_metadata", desc="The metadata of the attribute")]
    #[account(13, name="system_program", desc="The system program")]
    #[account(14, name="spl_associated_token_account", desc="The associated token account program")]
    #[account(15, name="spl_token", desc="The spl token program")]
    #[account(16, name="token_metadata_program", desc="The token metadata program")]
    #[account(17, name="sysvar_instructions", desc="Instructions sysvar account")]
    TransferOut(TransferOutArgs),

    #[account(0, writable, name = "constraint_model", desc = "Constraint model account")]
    #[account(1, writable, signer, name = "payer", desc = "Wallet paying for the transaction and new account, will be set as the creator of the constraint model")]
    #[account(2, signer, name = "update_authority", desc = "Update authority of the constraint model")]
    #[account(3, name = "system_program", desc = "System program")]
    #[account(4, name="sysvar_instructions", desc="Instructions sysvar account")]
    AddNoneConstraintToEscrowConstraintModel(AddNoneConstraintToEscrowConstraintModelArgs),

    #[account(0, writable, name = "constraint_model", desc = "Constraint model account")]
    #[account(1, writable, signer, name = "payer", desc = "Wallet paying for the transaction and new account, will be set as the creator of the constraint model")]
    #[account(2, signer, name = "update_authority", desc = "Update authority of the constraint model")]
    #[account(3, name = "collection_mint", desc = "Collection mint account")]
    #[account(4, name = "collection_mint_metadata", desc = "Collection mint metadata account")]
    #[account(5, name = "system_program", desc = "System program")]
    #[account(6, name="sysvar_instructions", desc="Instructions sysvar account")]
    AddCollectionConstraintToEscrowConstraintModel(AddCollectionConstraintToEscrowConstraintModelArgs),

    #[account(0, writable, name = "constraint_model", desc = "Constraint model account")]
    #[account(1, writable, signer, name = "payer", desc = "Wallet paying for the transaction and new account, will be set as the creator of the constraint model")]
    #[account(2, signer, name = "update_authority", desc = "Update authority of the constraint model")]
    #[account(3, name = "system_program", desc = "System program")]
    #[account(4, name="sysvar_instructions", desc="Instructions sysvar account")]
    AddTokensConstraintToEscrowConstraintModel(AddTokensConstraintToEscrowConstraintModelArgs),

    #[default_optional_accounts]
    #[account(0, writable, name = "constraint_model", desc = "Constraint model account")]
    #[account(1, writable, signer, name = "payer", desc = "Wallet paying for the transaction")]
    #[account(2, signer, name = "update_authority", desc = "Update authority of the constraint model")]
    #[account(3, name = "system_program", desc = "System program")]
    #[account(4, name="sysvar_instructions", desc="Instructions sysvar account")]
    RemoveConstraintFromEscrowConstraintModel(RemoveConstraintFromEscrowConstraintModelArgs),

    #[account(0, writable, name = "constraint_model", desc = "Constraint model account")]
    #[account(1, writable, signer, name = "payer", desc = "Wallet paying for the transaction and new account, will be set as the creator of the constraint model")]
    #[account(2, signer, name = "update_authority", desc = "Update authority of the constraint model")]
    #[account(3, name = "system_program", desc = "System program")]
    #[account(4, name="sysvar_instructions", desc="Instructions sysvar account")]
    SetRoyalties(SetRoyaltiesArgs),

    #[account(0, writable, name = "constraint_model", desc = "Constraint model account")]
    #[account(1, writable, signer, name = "payer", desc = "Wallet paying for the transaction and new account, will be set as the creator of the constraint model")]
    #[account(2, signer, name = "update_authority", desc = "Update authority of the constraint model")]
    #[account(3, name = "destination", desc = "The account to withdraw the royalties to")]
    #[account(4, name = "system_program", desc = "System program")]
    #[account(5, name="sysvar_instructions", desc="Instructions sysvar account")]
    WithdrawRoyalties(WithdrawRoyaltiesArgs),

    #[account(0, writable, name = "constraint_model", desc = "Constraint model account")]
    #[account(1, writable, signer, name = "payer", desc = "Wallet paying for the transaction and new account, will be set as the creator of the constraint model")]
    #[account(2, signer, name = "update_authority", desc = "Update authority of the constraint model")]
    #[account(3, name = "first_creator", desc = "First creator account")]
    #[account(4, name = "system_program", desc = "System program")]
    #[account(5, name="sysvar_instructions", desc="Instructions sysvar account")]
    AddFirstCreatorConstraintToEscrowConstraintModel(AddFirstCreatorConstraintToEscrowConstraintModelArgs),
}
