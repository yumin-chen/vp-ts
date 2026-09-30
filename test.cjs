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
  assert.equal(ksuid.to_string(), base62)

  const bytes = ksuid.bytes()
  assert.equal(Buffer.isBuffer(bytes), true)
  assert.equal(bytes.length, 20)

  const payload = ksuid.payload()
  assert.equal(Buffer.isBuffer(payload), true)
  assert.equal(payload.length, 16)

  const tsSec = ksuid.timestampSeconds()
  assert.equal(typeof tsSec, 'number')
  assert.ok(tsSec > 1600000000)
  assert.equal(ksuid.timestamp_seconds(), tsSec)

  const tsMs = ksuid.timestamp()
  assert.equal(typeof tsMs, 'number')
  assert.equal(tsMs, tsSec * 1000)
})

test('Creating Ksuids', async () => {
  const { Ksuid } = await import('./index.js')

  const ksuidNow = Ksuid.now()
  assert.ok(ksuidNow)

  const payload = Buffer.alloc(16, 12)
  const ksuidNew = Ksuid.new(1621627443, payload)
  assert.equal(ksuidNew.timestampSeconds(), 1621627443)
  assert.deepEqual(ksuidNew.payload(), payload)

  const base62 = '1srOrx2ZWZBpBUvZwXKQmoEYga2'
  const kFromB62 = Ksuid.fromBase62(base62)
  const kFromStr = Ksuid.fromStr(base62)
  assert.equal(kFromB62.toString(), base62)
  assert.equal(kFromStr.toString(), base62)
  assert.equal(Ksuid.from_base62(base62).toString(), base62)
  assert.equal(Ksuid.from_str(base62).toString(), base62)

  const bytes = kFromB62.bytes()
  const kFromBytes = Ksuid.fromBytes(bytes)
  assert.equal(kFromBytes.toString(), base62)
  assert.equal(Ksuid.from_bytes(bytes).toString(), base62)

  const kFromSec = Ksuid.fromSeconds(1621627443, payload)
  assert.equal(kFromSec.timestampSeconds(), 1621627443)
  assert.equal(Ksuid.from_seconds(1621627443, payload).timestampSeconds(), 1621627443)
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
  assert.equal(ksuid2.compare(ksuid1), 1)
  assert.equal(ksuid1.compare(ksuid1), 0)
})

test('KsuidMs functionality', async () => {
  const { KsuidMs } = await import('./index.js')

  const kms = KsuidMs.now()
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
