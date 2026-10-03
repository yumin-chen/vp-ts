import { describe, it, expect } from 'vite-plus/test';
import { createSign, sign } from '../index.js';

describe('Signature module', () => {
  it('createSign and sign class', () => {
    const s = createSign('Ed25519');
    s.update('test payload');
    expect(s).toBeDefined();
  });
});
