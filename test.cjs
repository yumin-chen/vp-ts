const assert = require('node:assert/strict')
const test = require('node:test')

test('Ksuid native addon functions and Crockford Base32 options object', async () => {
  const { Ksuid, shuffleAlphabet } = await import('./index.js')
  Ksuid.setDefaultEncoding('base62')
  const ksuid = Ksuid.now()
  assert.equal(typeof ksuid.toString(), 'string')
  assert.equal(ksuid.bytes().length, 20)

  const base62 = ksuid.toString()
  assert.equal(ksuid.toString({ enc: 'base62' }), base62)

  const crockford = ksuid.toCrockfordBase32()
  assert.equal(ksuid.toString({ enc: 'base32' }), crockford)
  assert.equal(ksuid.toString({ enc: 'crockford' }), crockford)

  const customAlpha = shuffleAlphabet('0123456789ABCDEFGHJKMNPQRSTVWXYZ', 'test-seed')
  assert.equal(
    ksuid.toString({ enc: 'base32', alphabet: customAlpha }),
    ksuid.toCrockfordBase32(customAlpha)
  )
})
