/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { Context, Serializer } from '@metaplex-foundation/umi';

export enum Key {
  Uninitialized,
  EscrowConstraintModel,
  Trifle,
}

export type KeyArgs = Key;

export function getKeySerializer(
  context: Pick<Context, 'serializer'>
): Serializer<KeyArgs, Key> {
  const s = context.serializer;
  return s.enum<Key>(Key, { description: 'Key' }) as Serializer<KeyArgs, Key>;
}
