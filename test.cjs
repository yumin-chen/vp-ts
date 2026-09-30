const assert = require('node:assert/strict')
const test = require('node:test')

test('ksuid functions', async () => {
  const {
    newKsuid,
    ksuidFromBase62,
    ksuidToBytes,
    ksuidTimestampSeconds,
  } = await import('./index.js')

  const id = newKsuid()
  assert.equal(typeof id, 'string')
  assert.equal(id.length, 27)

  const parsed = ksuidFromBase62(id)
  assert.equal(parsed, id)

  const bytes = ksuidToBytes(id)
  assert.equal(bytes.length, 20)

  const timestamp = ksuidTimestampSeconds(id)
  assert.ok(typeof timestamp === 'number' || typeof timestamp === 'bigint')
  assert.ok(Number(timestamp) > 1600000000)
})
