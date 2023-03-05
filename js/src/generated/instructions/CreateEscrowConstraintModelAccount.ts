/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@solana/web3.js'
import {
  CreateEscrowConstraintModelAccountArgs,
  createEscrowConstraintModelAccountArgsBeet,
} from '../types/CreateEscrowConstraintModelAccountArgs'

/**
 * @category Instructions
 * @category CreateEscrowConstraintModelAccount
 * @category generated
 */
export type CreateEscrowConstraintModelAccountInstructionArgs = {
  createEscrowConstraintModelAccountArgs: CreateEscrowConstraintModelAccountArgs
}
/**
 * @category Instructions
 * @category CreateEscrowConstraintModelAccount
 * @category generated
 */
export const CreateEscrowConstraintModelAccountStruct =
  new beet.FixableBeetArgsStruct<
    CreateEscrowConstraintModelAccountInstructionArgs & {
      instructionDiscriminator: number
    }
  >(
    [
      ['instructionDiscriminator', beet.u8],
      [
        'createEscrowConstraintModelAccountArgs',
        createEscrowConstraintModelAccountArgsBeet,
      ],
    ],
    'CreateEscrowConstraintModelAccountInstructionArgs'
  )
/**
 * Accounts required by the _CreateEscrowConstraintModelAccount_ instruction
 *
 * @property [_writable_] escrowConstraintModel Constraint model account
 * @property [_writable_, **signer**] payer Wallet paying for the transaction and new account, will be set as the creator of the constraint model
 * @property [] updateAuthority Update authority of the constraint model
 * @category Instructions
 * @category CreateEscrowConstraintModelAccount
 * @category generated
 */
export type CreateEscrowConstraintModelAccountInstructionAccounts = {
  escrowConstraintModel: web3.PublicKey
  payer: web3.PublicKey
  updateAuthority: web3.PublicKey
  systemProgram?: web3.PublicKey
}

export const createEscrowConstraintModelAccountInstructionDiscriminator = 0

/**
 * Creates a _CreateEscrowConstraintModelAccount_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category CreateEscrowConstraintModelAccount
 * @category generated
 */
export function createCreateEscrowConstraintModelAccountInstruction(
  accounts: CreateEscrowConstraintModelAccountInstructionAccounts,
  args: CreateEscrowConstraintModelAccountInstructionArgs,
  programId = new web3.PublicKey('trifMWutwBxkSuatmpPVnEe7NoE3BJKgjVi8sSyoXWX')
) {
  const [data] = CreateEscrowConstraintModelAccountStruct.serialize({
    instructionDiscriminator:
      createEscrowConstraintModelAccountInstructionDiscriminator,
    ...args,
  })
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.escrowConstraintModel,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.payer,
      isWritable: true,
      isSigner: true,
    },
    {
      pubkey: accounts.updateAuthority,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.systemProgram ?? web3.SystemProgram.programId,
      isWritable: false,
      isSigner: false,
    },
  ]

  const ix = new web3.TransactionInstruction({
    programId,
    keys,
    data,
  })
  return ix
}
