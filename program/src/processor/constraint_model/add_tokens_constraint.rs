use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    sysvar::instructions::{get_instruction_relative, load_current_index_checked},
};

use crate::{
    instruction::AddTokensConstraintToEscrowConstraintModelArgs,
    processor::constraint_model::add_constraint_to_escrow_constraint_model,
    state::escrow_constraints::{EscrowConstraint, EscrowConstraintType},
    util::is_creation_instruction,
};

pub fn add_tokens_constraint_to_escrow_constraint_model(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: AddTokensConstraintToEscrowConstraintModelArgs,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    accounts_iter.next(); // skip the escrow constraint model
    accounts_iter.next(); // skip the payer
    accounts_iter.next(); // skip the update authority
    accounts_iter.next(); // skip the system program
    let sysvar_instruction_info = next_account_info(accounts_iter)?;

    let constraint = EscrowConstraint {
        constraint_type: EscrowConstraintType::tokens_from_slice(&args.tokens),
        token_limit: args.token_limit,
        transfer_effects: args.transfer_effects,
    };

    // Check if the previous instruction was a creation instruction, so we don't double-charge protocol fees.
    let mut creation_ix = false;
    if load_current_index_checked(sysvar_instruction_info)? > 0 {
        let prev_ix = get_instruction_relative(-1, sysvar_instruction_info)?;
        creation_ix = is_creation_instruction(*prev_ix.data.first().unwrap_or(&255));
    }

    add_constraint_to_escrow_constraint_model(
        program_id,
        accounts,
        !creation_ix,
        args.constraint_name,
        constraint,
    )
}
