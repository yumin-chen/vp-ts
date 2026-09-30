const assert = require('node:assert/strict')
const test = require('node:test')

test('Ksuid native addon functions and Crockford Base32 toString config', async () => {
  const { Ksuid, shuffleAlphabet } = await import('./index.js')
  const ksuid = Ksuid.now()
  assert.equal(typeof ksuid.toString(), 'string')
  assert.equal(ksuid.bytes().length, 20)
  assert.equal(ksuid.payloadBytes().length, 16)

  const base62 = ksuid.toString('base62')
  const parsed = Ksuid.fromBase62(base62)
  assert.equal(parsed.toString('base62'), base62)
  assert.equal(parsed.equals(ksuid), true)

  const crockford = ksuid.toCrockfordBase32()
  assert.equal(crockford.length, 32)
  const parsedCrockford = Ksuid.fromCrockfordBase32(crockford)
  assert.equal(parsedCrockford.equals(ksuid), true)

  Ksuid.setDefaultEncoding('crockford')
  assert.equal(ksuid.toString(), crockford)

  Ksuid.setDefaultEncoding('base62')
  assert.equal(ksuid.toString(), base62)
})
