/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as web3 from '@solana/web3.js'
import * as beet from '@metaplex-foundation/beet'
import * as beetSolana from '@metaplex-foundation/beet-solana'
export type AddTokensConstraintToEscrowConstraintModelArgs = {
  constraintName: string
  tokens: web3.PublicKey[]
  tokenLimit: beet.bignum
  transferEffects: number
}

/**
 * @category userTypes
 * @category generated
 */
export const addTokensConstraintToEscrowConstraintModelArgsBeet =
  new beet.FixableBeetArgsStruct<AddTokensConstraintToEscrowConstraintModelArgs>(
    [
      ['constraintName', beet.utf8String],
      ['tokens', beet.array(beetSolana.publicKey)],
      ['tokenLimit', beet.u64],
      ['transferEffects', beet.u16],
    ],
    'AddTokensConstraintToEscrowConstraintModelArgs'
  )
