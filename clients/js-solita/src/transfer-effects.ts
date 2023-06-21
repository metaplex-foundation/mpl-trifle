// @ts-nocheck until 5.2.1 is released in DefinitelyTyped.
import BN from 'bn.js';

export class TransferEffects {
  _bn: BN;

  constructor(n?: number) {
    this._bn = new BN(n || 0);
  }

  track(): boolean {
    return this._bn.testn(0);
  }

  withTrack(bool = true): this {
    this._bn.setn(0, bool);
    return this;
  }

  burn(): boolean {
    return this._bn.testn(1);
  }

  withBurn(bool = true): this {
    this._bn.setn(1, bool);
    return this;
  }

  freeze(): boolean {
    return this._bn.testn(2);
  }

  withFreeze(bool = true): this {
    this._bn.setn(2, bool);
    return this;
  }

  freezeParent(): boolean {
    return this._bn.testn(3);
  }

  withFreezeParent(bool = true): this {
    this._bn.setn(3, bool);
    return this;
  }

  authTransferIn(): boolean {
    return this._bn.testn(4);
  }

  withAuthTransferIn(bool = true): this {
    this._bn.setn(4, bool);
    return this;
  }

  authTransferOut(): boolean {
    return this._bn.testn(5);
  }

  withAuthTransferOut(bool = true): this {
    this._bn.setn(5, bool);
    return this;
  }

  toNumber(): number {
    return this._bn.toNumber();
  }
}
