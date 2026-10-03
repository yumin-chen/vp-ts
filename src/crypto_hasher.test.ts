import { describe, it, expect } from 'vite-plus/test';
import { Hash, createHash, hash, getHashes } from '../index.js';

describe('Crypto Hasher module', () => {
  it('getHashes lists supported algorithms', () => {
    const hashes = getHashes();
    expect(hashes).toContain('sha256');
    expect(hashes).toContain('sha512');
  });

  it('hash one-shot helper function', () => {
    const digest = hash('sha256', 'hello world', 'hex');
    expect(typeof digest).toBe('string');
    expect(digest.length).toBe(64);
  });

  it('Hash class & createHash', () => {
    const hasher = createHash('sha256');
    hasher.update('hello ');
    hasher.update('world');
    const digest = hasher.digest('hex');
    expect(digest).toBe(hash('sha256', 'hello world', 'hex'));
  });
});
