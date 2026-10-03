import { describe, it, expect } from 'vite-plus/test';
import { createSecretKey, createPublicKey, createPrivateKey } from '../index.js';

describe('KeyObject module', () => {
  it('creates secret key object', () => {
    const key = createSecretKey(Buffer.from('secret_material'));
    expect(key.type).toBe('secret');
    expect(key.export().toString()).toBe('secret_material');
  });

  it('creates public key object', () => {
    const key = createPublicKey(Buffer.from('pubkey_material'), 'rsa');
    expect(key.type).toBe('public');
    expect(key.asymmetricKeyType).toBe('rsa');
  });

  it('creates private key object', () => {
    const key = createPrivateKey(Buffer.from('privkey_material'), 'rsa');
    expect(key.type).toBe('private');
    expect(key.asymmetricKeyType).toBe('rsa');
  });
});
