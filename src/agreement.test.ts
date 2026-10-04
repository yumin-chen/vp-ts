import assert from 'node:assert/strict'
import test from 'node:test'
import pkg from '../index.js'

const {
  x25519GenerateKeypair,
  x25519DiffieHellman,
  createPublicKey,
  createPrivateKey,
  createSecretKey,
  encapsulate,
  decapsulate,
} = pkg

void test('X25519 keypair and diffie-hellman shared secret', () => {
  const alice = x25519GenerateKeypair()
  const bob = x25519GenerateKeypair()

  assert.equal(alice.length, 2)
  assert.equal(bob.length, 2)

  const alicePub = alice[0]
  const alicePriv = alice[1]
  const bobPub = bob[0]
  const bobPriv = bob[1]

  const secret1 = x25519DiffieHellman(alicePriv, bobPub)
  const secret2 = x25519DiffieHellman(bobPriv, alicePub)

  assert.equal(secret1.toString('hex'), secret2.toString('hex'))
})

void test('createPublicKey, createPrivateKey, createSecretKey', () => {
  const pub = createPublicKey('-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA...\n-----END PUBLIC KEY-----')
  assert.equal(pub.type, 'public')

  const priv = createPrivateKey('-----BEGIN PRIVATE KEY-----\nMIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQ...\n-----END PRIVATE KEY-----')
  assert.equal(priv.type, 'private')

  const secret = createSecretKey(Buffer.from('secret123'))
  assert.equal(secret.type, 'secret')
})

void test('encapsulate and decapsulate', () => {
  const pair = x25519GenerateKeypair()
  const pubKey = pair[0]
  const privKey = pair[1]

  const res = encapsulate(pubKey)
  assert.ok(res.sharedKey)
  assert.ok(res.ciphertext)

  const sharedDecapsulated = decapsulate(privKey, res.ciphertext)
  assert.equal(res.sharedKey.toString('hex'), sharedDecapsulated.toString('hex'))
})
