import { mplTokenMetadata } from '@metaplex-foundation/mpl-token-metadata';
import { UmiPlugin } from '@metaplex-foundation/umi';
import { createMplTrifleProgram } from './generated';

export const mplTrifle = (): UmiPlugin => ({
  install(umi) {
    umi.use(mplTokenMetadata());
    umi.programs.add(createMplTrifleProgram(), false);
  },
});
