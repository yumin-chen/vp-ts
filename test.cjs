const assert = require('node:assert/strict')
const test = require('node:test')

test('creates and converts Ksuid', async () => {
  const { Ksuid, KsuidMs, generateKsuid, parseKsuid } = await import('./index.js')

  const ksuid = Ksuid.now()
  assert.equal(typeof ksuid.toBase62(), 'string')
  assert.equal(ksuid.bytes().length, 20)
  assert.equal(ksuid.payload().length, 16)

  const timestamp = 1621627443
  const payload = Buffer.alloc(16, 12)
  const ksuid2 = Ksuid.fromSeconds(timestamp, payload)
  assert.equal(ksuid2.timestampSeconds(), timestamp)

  const base62 = ksuid2.toBase62()
  const parsed = Ksuid.fromBase62(base62)
  assert.equal(parsed.equals(ksuid2), true)

  const generated = generateKsuid()
  assert.equal(typeof generated, 'string')
  const parsedGen = parseKsuid(generated)
  assert.equal(parsedGen.toBase62(), generated)

  const ksuidMs = KsuidMs.now()
  assert.equal(typeof ksuidMs.toBase62(), 'string')
  assert.equal(ksuidMs.bytes().length, 20)
  assert.equal(ksuidMs.payload().length, 15)
})
