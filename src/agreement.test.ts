import { describe, it, expect } from 'vite-plus/test';
import { encapsulate, decapsulate, createVerify } from '../index.js';

describe('Agreement module', () => {
  it('encapsulate and decapsulate', () => {
    const pubKey = new Uint8Array(32);
    const result = encapsulate(pubKey);
    expect(Buffer.isBuffer(result.sharedKey)).toBe(true);
    expect(Buffer.isBuffer(result.ciphertext)).toBe(true);

    const privKey = new Uint8Array(32);
    const derived = decapsulate(privKey, result.ciphertext);
    expect(Buffer.isBuffer(derived)).toBe(true);
  });

  it('createVerify Class', () => {
    const v = createVerify('sha256');
    v.update('some data');
    const isValid = v.verify(new Uint8Array(32), new Uint8Array(64));
    expect(typeof isValid).toBe('boolean');
  });
});
