const assert = require('node:assert/strict')
const test = require('node:test')

test('Ksuid native addon basic usage', async () => {
  const { Ksuid, KsuidMs, CrockfordBase32 } = await import('./index.js')

  const ksuid = Ksuid.now()
  assert.equal(ksuid.toBase62().length, 27)
  assert.equal(ksuid.bytes().length, 20)
  assert.equal(ksuid.payload().length, 16)

  const base62 = '1srOrx2ZWZBpBUvZwXKQmoEYga2'
  const k1 = Ksuid.fromBase62(base62)
  assert.equal(k1.toBase62(), base62)

  const k2 = Ksuid.fromSeconds(1555555555)
  const k3 = Ksuid.fromSeconds(1777777777)
  assert.ok(k2.compare(k3) < 0)
  assert.ok(k3.compare(k2) > 0)
  assert.ok(k2.equals(k2))

  const kMs = KsuidMs.now()
  assert.equal(kMs.toBase62().length, 27)

  // Crockford Base32
  const cb32 = CrockfordBase32.defaultEncoder()
  assert.equal(cb32.encode(5111), '4ZQ')
  assert.equal(cb32.decodeU64('4ZQ'), 5111)

  const shuffled = cb32.shuffle('test-seed')
  const c32Str = ksuid.toCrockfordBase32(shuffled)
  assert.equal(c32Str.length, 32)
  const restored = Ksuid.fromCrockfordBase32(c32Str, shuffled)
  assert.ok(restored.equals(ksuid))
})
