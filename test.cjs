const assert = require('node:assert/strict')
const test = require('node:test')

test('ksuid and crockford base32 functions', async () => {
  const {
    Ksuid,
    newKsuid,
    ksuidFromBase62,
    ksuidToBytes,
    ksuidTimestampSeconds,
    crockfordEncode,
    crockfordDecode,
    encodeBytesCustom,
    decodeBytesCustom,
    ksuidToCrockford,
    ksuidFromCrockford,
  } = await import('./index.js')

  // Standard KSUID
  const id = newKsuid()
  assert.equal(typeof id, 'string')
  assert.equal(id.length, 27)

  const parsed = ksuidFromBase62(id)
  assert.equal(parsed, id)

  const bytes = ksuidToBytes(id)
  assert.equal(bytes.length, 20)

  const timestamp = ksuidTimestampSeconds(id)
  assert.ok(Number(timestamp) > 1600000000)

  // Crockford Base32 number encoding & decoding
  assert.equal(crockfordEncode(0), '0')
  assert.equal(crockfordEncode(65535), '1ZZZ')
  assert.equal(crockfordEncode(5111), '4ZQ')
  assert.equal(crockfordDecode('1zzz'), 65535)
  assert.equal(crockfordDecode('1ZZZ'), 65535)
  assert.equal(crockfordDecode('4ZQ'), 5111)

  // Custom alphabet shuffle
  const customAlphabet = 'FxnXM1kBN6cuhsAvjW3Co7l2RePyY8DwaU04Tzt9fHQrqSVKdpimLGIJOgb5ZE'
  const encodedCustom = crockfordEncode(65535, customAlphabet)
  const decodedCustom = crockfordDecode(encodedCustom, customAlphabet)
  assert.equal(decodedCustom, 65535)

  // Byte encoding & decoding with custom alphabet
  const testBytes = new Uint8Array([1, 2, 3, 4, 5])
  const encodedBytes = encodeBytesCustom(testBytes, customAlphabet)
  const decodedBytes = decodeBytesCustom(encodedBytes, customAlphabet)
  assert.deepEqual(Array.from(decodedBytes), [1, 2, 3, 4, 5])

  // KSUID to Crockford and back
  const crockfordKsuid = ksuidToCrockford(id, customAlphabet)
  const restoredKsuid = ksuidFromCrockford(crockfordKsuid, customAlphabet)
  assert.equal(restoredKsuid, id)

  // Ksuid with enc and timestampSize options ("32bit", "48bit", "64bit")
  const k32 = new Ksuid({ enc: 'base32', timestampSize: '32bit' })
  assert.ok(k32.toString().length > 0)
  assert.ok(k32.timestampSeconds() > 1600000000)

  const k48 = new Ksuid({ enc: 'base32', timestampSize: '48bit' })
  assert.ok(k48.toString().length > 0)
  assert.ok(k48.timestampMillis() > 1600000000000)

  const k64 = new Ksuid({ enc: 'base32', timestampSize: '64bit' })
  assert.ok(k64.toString().length > 0)
  assert.ok(k64.timestampMillis() > 1600000000000)
})
