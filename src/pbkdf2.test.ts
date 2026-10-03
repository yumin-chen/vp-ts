import { test, expect } from 'vite-plus/test'
import { pbkdf2Sync, pbkdf2, Pbkdf2 } from '../index.js'

test('PBKDF2 Module - pbkdf2Sync matching expected SHA256 output', () => {
  const password = Buffer.from('password')
  const salt = Buffer.from('salt')
  const derived = pbkdf2Sync(password, salt, 1000, 32, 'sha256')
  expect(Buffer.isBuffer(derived)).toBe(true)
  expect(derived.length).toBe(32)
  expect(derived.toString('hex')).toBe('632c2812e46d4604102ba7618e9d6d7d2f8128f6266b4a03264d2a0460b7dcb3')
})

test('PBKDF2 Module - async pbkdf2 matching pbkdf2Sync', async () => {
  const password = Buffer.from('secretPass')
  const salt = Buffer.from('randomSalt123')
  const syncDerived = pbkdf2Sync(password, salt, 2000, 64, 'sha512')
  const asyncDerived = await pbkdf2(password, salt, 2000, 64, 'sha512')
  expect(asyncDerived.toString('hex')).toBe(syncDerived.toString('hex'))
})

test('PBKDF2 Module - Pbkdf2 class instance', async () => {
  const instance = new Pbkdf2(500, 16, 'sha256')
  const password = Buffer.from('pass')
  const salt = Buffer.from('salt')
  const syncRes = instance.deriveSync(password, salt)
  const asyncRes = await instance.derive(password, salt)
  expect(syncRes.length).toBe(16)
  expect(asyncRes.toString('hex')).toBe(syncRes.toString('hex'))
})

test('PBKDF2 Module - throw on invalid iterations or unsupported digest', () => {
  const password = Buffer.from('pass')
  const salt = Buffer.from('salt')
  expect(() => pbkdf2Sync(password, salt, 0, 32, 'sha256')).toThrow()
  expect(() => pbkdf2Sync(password, salt, 1000, 32, 'unsupported_digest')).toThrow()
})
