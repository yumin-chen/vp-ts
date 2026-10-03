import { describe, it, expect } from 'vite-plus/test';
import { createECDH, createDiffieHellman } from '../index.js';

describe('ECDH module', () => {
  it('computes shared secret between Alice and Bob', () => {
    const alice = createECDH('prime256v1');
    alice.generateKeys();

    const bob = createECDH('prime256v1');
    bob.generateKeys();

    const aliceSecret = alice.computeSecret(bob.getPublicKey());
    const bobSecret = bob.computeSecret(alice.getPublicKey());

    expect(Buffer.isBuffer(aliceSecret)).toBe(true);
    expect(aliceSecret.toString('hex')).toBe(bobSecret.toString('hex'));
  });

  it('createDiffieHellman helper function', () => {
    const dh = createDiffieHellman('prime256v1');
    expect(Buffer.isBuffer(dh.getPublicKey())).toBe(true);
  });
});
