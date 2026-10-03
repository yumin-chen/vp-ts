import { describe, it, expect } from 'vite-plus/test';
import { Hmac, createHmac } from '../index.js';

describe('HMAC module', () => {
  it('createHmac sha256 hex digest', () => {
    const hmac = createHmac('sha256', 'a secret');
    hmac.update('hello world');
    const digest = hmac.digest('hex');
    expect(typeof digest).toBe('string');
    expect(digest.length).toBe(64);
  });

  it('Hmac class directly', () => {
    const hmac = new Hmac('sha512', 'key');
    hmac.update('data to be hashed');
    const buf = hmac.digest('buffer');
    expect(Buffer.isBuffer(buf)).toBe(true);
    expect(buf.length).toBe(64);
  });

  it('throws on update after digest', () => {
    const hmac = createHmac('sha256', 'key');
    hmac.digest('hex');
    expect(() => hmac.update('more')).toThrow();
  });
});
