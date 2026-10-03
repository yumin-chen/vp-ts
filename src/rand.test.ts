import { describe, it, expect } from 'vite-plus/test';
import { randomBytes, randomFillSync, randomInt, randomUuid } from '../index.js';

describe('Rand module', () => {
  it('randomBytes generates requested buffer size', () => {
    const buf = randomBytes(32);
    expect(Buffer.isBuffer(buf)).toBe(true);
    expect(buf.length).toBe(32);
  });

  it('randomFillSync fills typed array', () => {
    const arr = new Uint8Array(16);
    randomFillSync(arr, 0, 16);
    expect(arr.some((b) => b !== 0)).toBe(true);
  });

  it('randomInt produces number within range', () => {
    const val = randomInt(1, 10);
    expect(val).toBeGreaterThanOrEqual(1);
    expect(val).toBeLessThan(10);
  });

  it('randomUuid generates valid UUID v4', () => {
    const uuid = randomUuid();
    expect(typeof uuid).toBe('string');
    expect(uuid.length).toBe(36);
  });
});
