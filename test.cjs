const assert = require('node:assert/strict')
const test = require('node:test')

test('Ksuid native addon functions and KsuidMs millisecond precision', async () => {
  const { Ksuid, KsuidMs } = await import('./index.js')
  Ksuid.setDefaultEncoding('base62')
  const ksuid = Ksuid.now()
  assert.equal(typeof ksuid.toString(), 'string')
  assert.equal(ksuid.bytes().length, 20)

  const ksuidMs = KsuidMs.now()
  const tsMs = ksuidMs.timestampMs()
  assert.ok(tsMs > 1600000000000)
  assert.equal(ksuidMs.bytes().length, 20)

  const base62 = ksuidMs.toString()
  const parsedMs = KsuidMs.fromBase62(base62)
  assert.equal(parsedMs.timestampMs(), tsMs)
  assert.equal(parsedMs.equals(ksuidMs), true)
})
