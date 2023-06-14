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
export type CreateTrifleAccountInstructionAccounts = {
  /** Escrow account */
  escrow: PublicKey;
  /** Metadata account */
  metadata: PublicKey;
  /** Mint account */
  mint: PublicKey;
  /** Token account (base token) */
  tokenAccount: PublicKey;
  /** Edition account */
  edition: PublicKey;
  /** Trifle account */
  trifleAccount: PublicKey;
  /** Trifle Authority - the account that can sign transactions for the trifle account */
  trifleAuthority: Signer;
  /** Escrow constraint model */
  constraintModel: PublicKey;
  /** Wallet paying for the transaction */
  payer?: Signer;
  /** Token Metadata program */
  tokenMetadataProgram?: PublicKey;
  /** System program */
  systemProgram?: PublicKey;
  /** Instructions sysvar account */
  sysvarInstructions?: PublicKey;
};

// Arguments.
export type CreateTrifleAccountInstructionData = { discriminator: number };

export type CreateTrifleAccountInstructionDataArgs = {};

export function getCreateTrifleAccountInstructionDataSerializer(
  context: Pick<Context, 'serializer'>
): Serializer<
  CreateTrifleAccountInstructionDataArgs,
  CreateTrifleAccountInstructionData
> {
  const s = context.serializer;
  return mapSerializer<
    CreateTrifleAccountInstructionDataArgs,
    CreateTrifleAccountInstructionData,
    CreateTrifleAccountInstructionData
  >(
    s.struct<CreateTrifleAccountInstructionData>([['discriminator', s.u8()]], {
      description: 'CreateTrifleAccountInstructionData',
    }),
    (value) =>
      ({ ...value, discriminator: 1 } as CreateTrifleAccountInstructionData)
  ) as Serializer<
    CreateTrifleAccountInstructionDataArgs,
    CreateTrifleAccountInstructionData
  >;
}

// Instruction.
export function createTrifleAccount(
  context: Pick<Context, 'serializer' | 'programs' | 'payer'>,
  input: CreateTrifleAccountInstructionAccounts
): TransactionBuilder {
  const signers: Signer[] = [];
  const keys: AccountMeta[] = [];

  // Program ID.
  const programId = context.programs.getPublicKey(
    'mplTrifle',
    'trifMWutwBxkSuatmpPVnEe7NoE3BJKgjVi8sSyoXWX'
  );

  // Resolved accounts.
  const escrowAccount = input.escrow;
  const metadataAccount = input.metadata;
  const mintAccount = input.mint;
  const tokenAccountAccount = input.tokenAccount;
  const editionAccount = input.edition;
  const trifleAccountAccount = input.trifleAccount;
  const trifleAuthorityAccount = input.trifleAuthority;
  const constraintModelAccount = input.constraintModel;
  const payerAccount = input.payer ?? context.payer;
  const tokenMetadataProgramAccount = input.tokenMetadataProgram ?? {
    ...context.programs.getPublicKey(
      'mplTokenMetadata',
      'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s'
    ),
    isWritable: false,
  };
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

  // Escrow.
  keys.push({
    pubkey: escrowAccount,
    isSigner: false,
    isWritable: isWritable(escrowAccount, true),
  });

  // Metadata.
  keys.push({
    pubkey: metadataAccount,
    isSigner: false,
    isWritable: isWritable(metadataAccount, true),
  });

  // Mint.
  keys.push({
    pubkey: mintAccount,
    isSigner: false,
    isWritable: isWritable(mintAccount, false),
  });

  // Token Account.
  keys.push({
    pubkey: tokenAccountAccount,
    isSigner: false,
    isWritable: isWritable(tokenAccountAccount, true),
  });

  // Edition.
  keys.push({
    pubkey: editionAccount,
    isSigner: false,
    isWritable: isWritable(editionAccount, false),
  });

  // Trifle Account.
  keys.push({
    pubkey: trifleAccountAccount,
    isSigner: false,
    isWritable: isWritable(trifleAccountAccount, true),
  });

  // Trifle Authority.
  signers.push(trifleAuthorityAccount);
  keys.push({
    pubkey: trifleAuthorityAccount.publicKey,
    isSigner: true,
    isWritable: isWritable(trifleAuthorityAccount, false),
  });

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

  // Token Metadata Program.
  keys.push({
    pubkey: tokenMetadataProgramAccount,
    isSigner: false,
    isWritable: isWritable(tokenMetadataProgramAccount, false),
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
  const data = getCreateTrifleAccountInstructionDataSerializer(
    context
  ).serialize({});

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}