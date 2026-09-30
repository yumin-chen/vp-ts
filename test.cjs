const assert = require('node:assert/strict')
const test = require('node:test')

test('Converting Ksuids', async () => {
  const { Ksuid } = await import('./index.js')
  const ksuid = Ksuid.now()

  const base62 = ksuid.toString()
  assert.equal(typeof base62, 'string')
  assert.equal(base62.length, 27)
  assert.equal(ksuid.toBase62(), base62)
  assert.equal(ksuid.to_base62(), base62)

  const bytes = ksuid.bytes()
  assert.equal(Buffer.isBuffer(bytes), true)
  assert.equal(bytes.length, 20)

  const payload = ksuid.payload()
  assert.equal(Buffer.isBuffer(payload), true)
  assert.equal(payload.length, 16)

  const tsSec = ksuid.timestampSeconds()
  assert.equal(typeof tsSec, 'number')
  assert.ok(tsSec > 1600000000)

  const tsMs = ksuid.timestamp()
  assert.equal(typeof tsMs, 'number')
  assert.equal(tsMs, tsSec * 1000)
})

test('Creating Ksuids with base32 options and custom alphabet', async () => {
  const { Ksuid, encodeCrockford, decodeCrockford } = await import('./index.js')

  const ksuidB32 = Ksuid.now(null, { enc: 'base32' })
  const b32Str = ksuidB32.toString()
  assert.equal(typeof b32Str, 'string')
  assert.equal(b32Str.length, 32) // 20 bytes * 8 / 5 = 32 chars

  const parsedB32 = Ksuid.fromBase32(b32Str, { enc: 'base32' })
  assert.equal(parsedB32.toBase32(), b32Str)
  assert.equal(parsedB32.toString(), b32Str)

  // Crockford u64 encode/decode
  const encodedVal = encodeCrockford(5111)
  assert.equal(encodedVal, '4ZQ')
  const decodedVal = decodeCrockford('4ZQ')
  assert.equal(decodedVal, 5111)

  // Custom alphabet
  const customAlpha = '0123456789abcdefghijklmnopqrstuv'
  const customKsuid = Ksuid.now(null, { enc: 'base32', alphabet: customAlpha })
  const customStr = customKsuid.toString()
  assert.equal(customStr, customStr.toLowerCase())
})

test('Compare and order Ksuids', async () => {
  const { Ksuid } = await import('./index.js')

  const ksuid1 = Ksuid.fromSeconds(1555555555)
  const ksuid2 = Ksuid.fromSeconds(1777777777)

  assert.ok(ksuid1.lt(ksuid2))
  assert.ok(ksuid1.lte(ksuid2))
  assert.ok(ksuid1.equals(ksuid1))
  assert.ok(ksuid2.gt(ksuid1))
  assert.ok(ksuid2.gte(ksuid1))
  assert.equal(ksuid1.compare(ksuid2), -1)
})

test('KsuidMs functionality with options', async () => {
  const { KsuidMs } = await import('./index.js')

  const kms = KsuidMs.now(null, { timestampSize: '64bit', enc: 'base32' })
  assert.ok(kms.toString())
  assert.equal(kms.bytes().length, 20)
  assert.equal(kms.payload().length, 15)

  const kms1 = KsuidMs.fromMillis(1555555555000)
  const kms2 = KsuidMs.fromMillis(1777777777000)

  assert.ok(kms1.lt(kms2))
  assert.equal(kms1.compare(kms2), -1)
  assert.equal(kms1.timestampSeconds(), 1555555555)
  assert.equal(kms1.timestampMillis(), 1555555555000)
})
