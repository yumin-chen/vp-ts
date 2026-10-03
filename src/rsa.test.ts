import { describe, it, expect } from 'vite-plus/test';
import { generateKeyPair, generateKeyPairSync } from '../index.js';

describe('RSA module', () => {
  it('generateKeyPairSync generates RSA PEM key pair', () => {
    const pair = generateKeyPairSync('rsa', 1024);
    expect(pair.publicKey).toContain('-----BEGIN PUBLIC KEY-----');
    expect(pair.privateKey).toContain('-----BEGIN PRIVATE KEY-----');
  }, 15000);

  it('async generateKeyPair', async () => {
    const pair = await generateKeyPair('rsa', 1024);
    expect(pair.publicKey).toContain('-----BEGIN PUBLIC KEY-----');
    expect(pair.privateKey).toContain('-----BEGIN PRIVATE KEY-----');
  }, 15000);
});
