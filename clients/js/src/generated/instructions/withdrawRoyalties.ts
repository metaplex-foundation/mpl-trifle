/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  AccountMeta,
  Context,
  PublicKey,
  Serializer,
  Signer,
  TransactionBuilder,
  checkForIsWritableOverride as isWritable,
  mapSerializer,
  publicKey,
  transactionBuilder,
} from '@metaplex-foundation/umi';

// Accounts.
export type WithdrawRoyaltiesInstructionAccounts = {
  /** Constraint model account */
  constraintModel: PublicKey;
  /** Wallet paying for the transaction and new account, will be set as the creator of the constraint model */
  payer?: Signer;
  /** Update authority of the constraint model */
  updateAuthority: Signer;
  /** The account to withdraw the royalties to */
  destination: PublicKey;
  /** System program */
  systemProgram?: PublicKey;
  /** Instructions sysvar account */
  sysvarInstructions?: PublicKey;
};

// Arguments.
export type WithdrawRoyaltiesInstructionData = {
  discriminator: number;
  name: string;
};

export type WithdrawRoyaltiesInstructionDataArgs = { name: string };

export function getWithdrawRoyaltiesInstructionDataSerializer(
  context: Pick<Context, 'serializer'>
): Serializer<
  WithdrawRoyaltiesInstructionDataArgs,
  WithdrawRoyaltiesInstructionData
> {
  const s = context.serializer;
  return mapSerializer<
    WithdrawRoyaltiesInstructionDataArgs,
    WithdrawRoyaltiesInstructionData,
    WithdrawRoyaltiesInstructionData
  >(
    s.struct<WithdrawRoyaltiesInstructionData>(
      [
        ['discriminator', s.u8()],
        ['name', s.string()],
      ],
      { description: 'WithdrawRoyaltiesInstructionData' }
    ),
    (value) =>
      ({ ...value, discriminator: 9 } as WithdrawRoyaltiesInstructionData)
  ) as Serializer<
    WithdrawRoyaltiesInstructionDataArgs,
    WithdrawRoyaltiesInstructionData
  >;
}

// Instruction.
export function withdrawRoyalties(
  context: Pick<Context, 'serializer' | 'programs' | 'payer'>,
  input: WithdrawRoyaltiesInstructionAccounts &
    WithdrawRoyaltiesInstructionDataArgs
): TransactionBuilder {
  const signers: Signer[] = [];
  const keys: AccountMeta[] = [];

  // Program ID.
  const programId = context.programs.getPublicKey(
    'mplTrifle',
    'trifMWutwBxkSuatmpPVnEe7NoE3BJKgjVi8sSyoXWX'
  );

  // Resolved accounts.
  const constraintModelAccount = input.constraintModel;
  const payerAccount = input.payer ?? context.payer;
  const updateAuthorityAccount = input.updateAuthority;
  const destinationAccount = input.destination;
  const systemProgramAccount = input.systemProgram ?? {
    ...context.programs.getPublicKey(
      'splSystem',
      '11111111111111111111111111111111'
    ),
    isWritable: false,
  };
  const sysvarInstructionsAccount =
    input.sysvarInstructions ??
    publicKey('Sysvar1nstructions1111111111111111111111111');

  // Constraint Model.
  keys.push({
    pubkey: constraintModelAccount,
    isSigner: false,
    isWritable: isWritable(constraintModelAccount, true),
  });

  // Payer.
  signers.push(payerAccount);
  keys.push({
    pubkey: payerAccount.publicKey,
    isSigner: true,
    isWritable: isWritable(payerAccount, true),
  });

  // Update Authority.
  signers.push(updateAuthorityAccount);
  keys.push({
    pubkey: updateAuthorityAccount.publicKey,
    isSigner: true,
    isWritable: isWritable(updateAuthorityAccount, false),
  });

  // Destination.
  keys.push({
    pubkey: destinationAccount,
    isSigner: false,
    isWritable: isWritable(destinationAccount, false),
  });

  // System Program.
  keys.push({
    pubkey: systemProgramAccount,
    isSigner: false,
    isWritable: isWritable(systemProgramAccount, false),
  });

  // Sysvar Instructions.
  keys.push({
    pubkey: sysvarInstructionsAccount,
    isSigner: false,
    isWritable: isWritable(sysvarInstructionsAccount, false),
  });

  // Data.
  const data =
    getWithdrawRoyaltiesInstructionDataSerializer(context).serialize(input);

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
