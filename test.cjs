const assert = require('node:assert/strict')
const test = require('node:test')

test('Ksuid native addon Base36 and Crockford encoding options', async () => {
  const { Ksuid, KsuidMs, shuffleAlphabet } = await import('./index.js')
  Ksuid.setDefaultEncoding('base62')
  const ksuid = Ksuid.now()
  assert.equal(typeof ksuid.toString(), 'string')
  assert.equal(ksuid.bytes().length, 20)

  // Base36 test
  const b36 = ksuid.toBase36()
  assert.equal(b36.length, 31)
  const parsedB36 = Ksuid.fromBase36(b36)
  assert.equal(parsedB36.equals(ksuid), true)

  // Options object
  assert.equal(ksuid.toString({ enc: 'base36' }), b36)

  // Custom alphabet
  const customAlpha = shuffleAlphabet('0123456789abcdefghijklmnopqrstuvwxyz', 'b36-test')
  const customB36 = ksuid.toBase36(customAlpha)
  const parsedCustom = Ksuid.fromBase36(customB36, customAlpha)
  assert.equal(parsedCustom.equals(ksuid), true)
})
