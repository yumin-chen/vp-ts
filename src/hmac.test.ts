import { test, expect } from 'vite-plus/test'
import { createHmac, Hmac } from '../index.js'

test('HMAC Module - sha256 hex', () => {
  const key = Buffer.from('a secret')
  const hmac = createHmac('sha256', key)
  hmac.update(Buffer.from('hello world'))
  const digest = hmac.digest('hex') as string
  expect(digest).toBe('322d4e7e52c59af88c8290fdbf52579a32d1e30f1d8b0a34808771e95cc88ab3')
})

test('HMAC Module - buffer digest', () => {
  const key = Buffer.from('secret')
  const hmac = new Hmac('sha256', key)
  hmac.update(Buffer.from('test data'))
  const buf = hmac.digest() as Buffer
  expect(Buffer.isBuffer(buf)).toBe(true)
  expect(buf.length).toBe(32)
})

test('HMAC Module - throw after digest', () => {
  const key = Buffer.from('secret')
  const hmac = createHmac('sha256', key)
  hmac.update(Buffer.from('data'))
  hmac.digest('hex')
  expect(() => hmac.update(Buffer.from('more'))).toThrow()
  expect(() => hmac.digest('hex')).toThrow()
})

test('HMAC Module - unsupported algorithm', () => {
  const key = Buffer.from('secret')
  expect(() => createHmac('invalid_algo', key)).toThrow()
})
