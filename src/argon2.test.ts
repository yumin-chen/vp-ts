import { describe, it, expect } from 'vite-plus/test';
import { argon2Hash, argon2HashSync, argon2HashRaw, argon2HashRawSync, argon2Verify, argon2VerifySync, argon2ParseOptions, Algorithm } from '../index.js';

describe('Argon2 module', () => {
  it('argon2HashSync and argon2VerifySync', () => {
    const pwd = 'password123';
    const hashed = argon2HashSync(pwd);
    expect(typeof hashed).toBe('string');
    expect(hashed).toContain('$argon2id$');

    const isValid = argon2VerifySync(hashed, pwd);
    expect(isValid).toBe(true);

    const isInvalid = argon2VerifySync(hashed, 'wrongpassword');
    expect(isInvalid).toBe(false);
  });

  it('async argon2Hash and argon2Verify', async () => {
    const pwd = 'secretpassword';
    const hashed = await argon2Hash(pwd);
    expect(typeof hashed).toBe('string');

    const isValid = await argon2Verify(hashed, pwd);
    expect(isValid).toBe(true);
  });

  it('argon2HashRawSync and argon2HashRaw', async () => {
    const pwd = 'my_raw_password';
    const rawSync = argon2HashRawSync(pwd, { memoryCost: 4096, timeCost: 1, parallelism: 1, salt: Buffer.from('1234567890123456') });
    expect(Buffer.isBuffer(rawSync)).toBe(true);

    const rawAsync = await argon2HashRaw(pwd, { memoryCost: 4096, timeCost: 1, parallelism: 1, salt: Buffer.from('1234567890123456') });
    expect(Buffer.isBuffer(rawAsync)).toBe(true);
    expect(rawSync.toString('hex')).toBe(rawAsync.toString('hex'));
  });

  it('argon2ParseOptions', () => {
    const pwd = 'test_parse_options';
    const hashed = argon2HashSync(pwd, { algorithm: Algorithm.Argon2i, memoryCost: 8192, timeCost: 2 });
    const parsed = argon2ParseOptions(hashed);
    expect(parsed.algorithm).toBe(Algorithm.Argon2i);
    expect(parsed.memoryCost).toBe(8192);
    expect(parsed.timeCost).toBe(2);
  });
});
