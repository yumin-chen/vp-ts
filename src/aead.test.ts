import { describe, it, expect } from 'vite-plus/test';
import { aeadEncrypt, aeadDecrypt } from '../index.js';

describe('AEAD module', () => {
  it('encrypt and decrypt with AES-256-GCM', () => {
    const key = new Uint8Array(32); // 256 bits
    const iv = new Uint8Array(12); // 96 bits
    const plaintext = Buffer.from('hello world');

    const ciphertext = aeadEncrypt('aes256gcm', key, iv, plaintext);
    expect(Buffer.isBuffer(ciphertext)).toBe(true);

    const decrypted = aeadDecrypt('aes256gcm', key, iv, ciphertext);
    expect(decrypted.toString()).toBe('hello world');
  });

  it('encrypt and decrypt with ChaCha20-Poly1305', () => {
    const key = new Uint8Array(32);
    const iv = new Uint8Array(12);
    const plaintext = Buffer.from('chacha test');

    const ciphertext = aeadEncrypt('chacha20poly1305', key, iv, plaintext);
    const decrypted = aeadDecrypt('chacha20poly1305', key, iv, ciphertext);
    expect(decrypted.toString()).toBe('chacha test');
  });
});
