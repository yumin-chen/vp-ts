import { test, expect } from 'vite-plus/test'
import { Hasher, createHash, hash, getHashes } from '../index.js'

test('Crypto Hasher Module - compute sha256 hash using Hasher class and createHash', () => {
  const hasher = createHash('sha256')
  hasher.update(Buffer.from('hello world'))
  const digest = hasher.digest('hex') as string
  expect(digest).toBe('b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9')
})

test('Crypto Hasher Module - method chaining update', () => {
  const digest = createHash('sha256').update('hello ').update('world').digest('hex') as string
  expect(digest).toBe('b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9')
})

test('Crypto Hasher Module - compute hash using one-shot hash function', () => {
  const resultHex = hash('sha256', Buffer.from('hello world'), 'hex') as string
  expect(resultHex).toBe('b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9')

  const resultBuf = hash('sha256', 'hello world', 'buffer') as Buffer
  expect(Buffer.isBuffer(resultBuf)).toBe(true)
  expect(resultBuf.toString('hex')).toBe('b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9')
})

test('Crypto Hasher Module - list available hashes', () => {
  const hashes = getHashes()
  expect(hashes).toContain('sha256')
  expect(hashes).toContain('sha512')
  expect(hashes).toContain('sha1')
})

test('Crypto Hasher Module - throw on unsupported hash algorithm', () => {
  expect(() => new Hasher('md5')).toThrow()
})
