const assert = require('node:assert/strict')
const test = require('node:test')

test('ksuid and crockford base32 functions', async () => {
  const {
    KsuidClient,
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
  const customAlphabet = 'ZYXWVUTSRQPONMKJHGFEDCBA98765432' // reversed alphabet
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

  // KsuidClient with constructor options & configurable toString()
  const defaultClient = new KsuidClient()
  assert.equal(defaultClient.toString().length, 27)

  const crockfordClient = new KsuidClient({
    encoding: 'crockford',
    alphabet: customAlphabet,
  })
  const crockfordString = crockfordClient.toString()
  assert.ok(crockfordString.length > 0)

  const parsedClient = KsuidClient.parse(crockfordString, {
    encoding: 'crockford',
    alphabet: customAlphabet,
  })
  assert.equal(parsedClient.toBase62(), crockfordClient.toBase62())
})
