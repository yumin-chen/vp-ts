const assert = require('node:assert/strict')
const test = require('node:test')

test('Ksuid native addon functions', async () => {
  const { Ksuid } = await import('./index.js')
  const ksuid = Ksuid.now()
  assert.equal(typeof ksuid.toString(), 'string')
  assert.equal(ksuid.bytes().length, 20)
  assert.equal(ksuid.payloadBytes().length, 16)

  const base62 = ksuid.toString()
  const parsed = Ksuid.fromBase62(base62)
  assert.equal(parsed.toString(), base62)
  assert.equal(parsed.equals(ksuid), true)
})
