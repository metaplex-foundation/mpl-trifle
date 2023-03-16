mod constraint_model;
mod royalties;
mod trifle;
pub use constraint_model::*;
pub use royalties::*;
pub use trifle::*;

use crate::instruction::TrifleInstruction;
use borsh::BorshDeserialize;

use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = TrifleInstruction::try_from_slice(input)?;

    match instruction {
        TrifleInstruction::CreateEscrowConstraintModelAccount(args) => {
            msg!("Instruction: Create Escrow Constraint Model Account");
            create_escrow_constraints_model_account(program_id, accounts, args)
        }
        TrifleInstruction::CreateTrifleAccount => {
            msg!("Instruction: Create Trifle Account");
            create_trifle_account(program_id, accounts)
        }
        TrifleInstruction::TransferIn(args) => {
            msg!("Instruction: Transfer In");
            transfer_in(program_id, accounts, args)
        }
        TrifleInstruction::TransferOut(args) => {
            msg!("Instruction: Transfer Out");
            transfer_out(program_id, accounts, args)
        }
        TrifleInstruction::AddNoneConstraintToEscrowConstraintModel(args) => {
            msg!("Instruction: Add None Constraint To Escrow Constraint Model");
            add_none_constraint_to_escrow_constraint_model(program_id, accounts, args)
        }
        TrifleInstruction::AddCollectionConstraintToEscrowConstraintModel(args) => {
            msg!("Instruction: Add Collection Constraint To Escrow Constraint Model");
            add_collection_constraint_to_escrow_constraint_model(program_id, accounts, args)
        }
        TrifleInstruction::AddTokensConstraintToEscrowConstraintModel(args) => {
            msg!("Instruction: Add Tokens Constraint To Escrow Constraint Model");
            add_tokens_constraint_to_escrow_constraint_model(program_id, accounts, args)
        }
        TrifleInstruction::RemoveConstraintFromEscrowConstraintModel(args) => {
            msg!("Instruction: Remove Constraint From Escrow Constraint Model");
            remove_constraint_from_escrow_constraint_model(program_id, accounts, args)
        }
        TrifleInstruction::SetRoyalties(args) => {
            msg!("Instruction: Set Royalties");
            set_royalties(program_id, accounts, args)
        }
        TrifleInstruction::WithdrawRoyalties(args) => {
            msg!("Instruction: Withdraw Royalties");
            withdraw_royalties(program_id, accounts, args)
        }
        TrifleInstruction::AddFirstCreatorConstraintToEscrowConstraintModel(args) => {
            msg!("Instruction: Add First Creator Constraint To Escrow Constraint Model");
            add_first_creator_constraint_to_escrow_constraint_model(program_id, accounts, args)
        }
    }
}
