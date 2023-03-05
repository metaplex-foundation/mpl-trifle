/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@solana/web3.js'
import {
  WithdrawRoyaltiesArgs,
  withdrawRoyaltiesArgsBeet,
} from '../types/WithdrawRoyaltiesArgs'

/**
 * @category Instructions
 * @category WithdrawRoyalties
 * @category generated
 */
export type WithdrawRoyaltiesInstructionArgs = {
  withdrawRoyaltiesArgs: WithdrawRoyaltiesArgs
}
/**
 * @category Instructions
 * @category WithdrawRoyalties
 * @category generated
 */
export const WithdrawRoyaltiesStruct = new beet.FixableBeetArgsStruct<
  WithdrawRoyaltiesInstructionArgs & {
    instructionDiscriminator: number
  }
>(
  [
    ['instructionDiscriminator', beet.u8],
    ['withdrawRoyaltiesArgs', withdrawRoyaltiesArgsBeet],
  ],
  'WithdrawRoyaltiesInstructionArgs'
)
/**
 * Accounts required by the _WithdrawRoyalties_ instruction
 *
 * @property [_writable_] constraintModel Constraint model account
 * @property [_writable_, **signer**] payer Wallet paying for the transaction and new account, will be set as the creator of the constraint model
 * @property [**signer**] updateAuthority Update authority of the constraint model
 * @property [] destination The account to withdraw the royalties to
 * @property [] sysvarInstructions Instructions sysvar account
 * @category Instructions
 * @category WithdrawRoyalties
 * @category generated
 */
export type WithdrawRoyaltiesInstructionAccounts = {
  constraintModel: web3.PublicKey
  payer: web3.PublicKey
  updateAuthority: web3.PublicKey
  destination: web3.PublicKey
  systemProgram?: web3.PublicKey
  sysvarInstructions: web3.PublicKey
}

export const withdrawRoyaltiesInstructionDiscriminator = 9

/**
 * Creates a _WithdrawRoyalties_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category WithdrawRoyalties
 * @category generated
 */
export function createWithdrawRoyaltiesInstruction(
  accounts: WithdrawRoyaltiesInstructionAccounts,
  args: WithdrawRoyaltiesInstructionArgs,
  programId = new web3.PublicKey('trifMWutwBxkSuatmpPVnEe7NoE3BJKgjVi8sSyoXWX')
) {
  const [data] = WithdrawRoyaltiesStruct.serialize({
    instructionDiscriminator: withdrawRoyaltiesInstructionDiscriminator,
    ...args,
  })
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.constraintModel,
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
      isSigner: true,
    },
    {
      pubkey: accounts.destination,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.systemProgram ?? web3.SystemProgram.programId,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.sysvarInstructions,
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
