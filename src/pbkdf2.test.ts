import { describe, it, expect } from 'vite-plus/test';
import { pbkdf2, pbkdf2Sync } from '../index.js';

describe('PBKDF2 module', () => {
  it('pbkdf2Sync', () => {
    const derived = pbkdf2Sync('secret', 'salt', 1000, 32, 'sha256');
    expect(Buffer.isBuffer(derived)).toBe(true);
    expect(derived.length).toBe(32);
  });

  it('async pbkdf2', async () => {
    const derivedSync = pbkdf2Sync('secret', 'salt', 1000, 32, 'sha256');
    const derivedAsync = await pbkdf2('secret', 'salt', 1000, 32, 'sha256');
    expect(Buffer.isBuffer(derivedAsync)).toBe(true);
    expect(derivedAsync.toString('hex')).toBe(derivedSync.toString('hex'));
  });
});
