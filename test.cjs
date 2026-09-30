const assert = require('node:assert/strict')
const test = require('node:test')

test('adds two numbers', async () => {
  const { add } = await import('./index.js')
  assert.equal(add(2, 3), 5)
})
