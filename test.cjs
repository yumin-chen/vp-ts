const assert = require('node:assert/strict')
const test = require('node:test')

test('creates and converts Ksuid', async () => {
  const {
    Ksuid,
    KsuidMs,
    generateKsuid,
    parseKsuid,
    encodeCrockfordBase32,
    decodeCrockfordBase32,
    shuffleAlphabet,
  } = await import('./index.js')

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

  // Crockford Base32
  const b32 = ksuid2.toBase32()
  assert.equal(b32.length, 32)
  const parsed32 = Ksuid.fromBase32(b32)
  assert.equal(parsed32.equals(ksuid2), true)

  assert.equal(encodeCrockfordBase32(5111), '4ZQ')
  assert.equal(decodeCrockfordBase32('4zq'), 5111)

  // Custom alphabet
  const stdAlphabet = '0123456789ABCDEFGHJKMNPQRSTVWXYZ'
  const customAlph = shuffleAlphabet(stdAlphabet, 42)
  const customEnc = ksuid2.toBase32(customAlph)
  const customParsed = Ksuid.fromBase32(customEnc, customAlph)
  assert.equal(customParsed.equals(ksuid2), true)

  const generated = generateKsuid()
  assert.equal(typeof generated, 'string')
  const parsedGen = parseKsuid(generated)
  assert.equal(parsedGen.toBase62(), generated)

  const ksuidMs = KsuidMs.now()
  assert.equal(typeof ksuidMs.toBase62(), 'string')
  assert.equal(ksuidMs.bytes().length, 20)
  assert.equal(ksuidMs.payload().length, 15)
})
