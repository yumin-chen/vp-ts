import assert from 'node:assert/strict'
import test from 'node:test'
import pkg from '../index.js'

const { generateKeyPairSync, validatePkcs8PrivateKey, validateSpkiPublicKey } = pkg

void test('validatePkcs8PrivateKey and validateSpkiPublicKey', () => {
  const { publicKey, privateKey } = generateKeyPairSync('rsa', { modulusLength: 2048 })

  assert.equal(validateSpkiPublicKey(publicKey), true)
  assert.equal(validatePkcs8PrivateKey(privateKey), true)
  assert.equal(validateSpkiPublicKey('invalid pem'), false)
  assert.equal(validatePkcs8PrivateKey('invalid pem'), false)
})
