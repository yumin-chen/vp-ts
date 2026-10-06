import assert from 'node:assert/strict'
import test from 'node:test'
import pkg from '../index.js'

const {
  randomBytes,
  randomFill,
  randomFillSync,
  randomInt,
  randomUUID,
  randomUUIDv7,
} = pkg

void test('randomBytes sync and async', async () => {
  const syncBuf = randomBytes(16)
  assert.equal(syncBuf.length, 16)

  const asyncBuf = await new Promise<Buffer>((resolve, reject) => {
    randomBytes(16, (err: Error | null, buf?: Buffer) => {
      if (err) reject(err)
      else resolve(buf!)
    })
  })
  assert.equal(asyncBuf.length, 16)
})

void test('randomBytes parameter validation', () => {
  assert.throws(() => randomBytes(-1), { code: 'ERR_OUT_OF_RANGE' })
  assert.throws(() => randomBytes(2147483648), { code: 'ERR_OUT_OF_RANGE' })
  assert.throws(() => (randomBytes as any)('invalid'), { code: 'ERR_INVALID_ARG_TYPE' })
})

void test('randomFill and randomFillSync', async () => {
  const buf1 = Buffer.alloc(10)
  randomFillSync(buf1, 2, 5)
  assert.equal(buf1.length, 10)

  const buf2 = Buffer.alloc(10)
  const filled = await new Promise<Buffer>((resolve, reject) => {
    randomFill(buf2, 2, 5, (err: Error | null, res?: Buffer) => {
      if (err) reject(err)
      else resolve(res!)
    })
  })
  assert.equal(filled.length, 10)
})

void test('randomInt sync and async', async () => {
  const syncVal = randomInt(1, 100)
  assert.ok(syncVal >= 1 && syncVal < 100)

  const asyncVal = await new Promise<number>((resolve, reject) => {
    randomInt(1, 100, (err: Error | null, n?: number) => {
      if (err) reject(err)
      else resolve(n!)
    })
  })
  assert.ok(asyncVal >= 1 && asyncVal < 100)
})

void test('randomUUID and randomUUIDv7', () => {
  const uuid4 = randomUUID()
  const uuid7 = randomUUIDv7()
  assert.equal(typeof uuid4, 'string')
  assert.equal(uuid4.length, 36)
  assert.equal(typeof uuid7, 'string')
  assert.equal(uuid7.length, 36)
})
