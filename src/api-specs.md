# `@lib/crypto` API Compatibility Specification

`@lib/crypto` provides a high-performance native Node.js `node:crypto` compatible implementation powered by NAPI-RS and open-source Rust cryptography primitives (`ring`, `rustls`, `argon2`, `aes-gcm`, `rsa`, `x25519-dalek`, `ed25519-dalek`, `x509-parser`).

## Module Mapping & Compatibility Matrix

| Standard `node:crypto` API                                | `@lib/crypto` Implementation            | Primary Backend Crate | Compatibility Status |
| :-------------------------------------------------------- | :-------------------------------------- | :-------------------- | :------------------- |
| `crypto.createHash(algorithm)`                            | `createHash(algorithm)`                 | `sha2`, `sha1`        | Supported            |
| `crypto.hash(algorithm, data, outputEncoding)`            | `hash(algorithm, data, outputEncoding)` | `sha2`, `sha1`        | Supported            |
| `crypto.getHashes()`                                      | `getHashes()`                           | Native                | Supported            |
| `crypto.createHmac(algorithm, key)`                       | `createHmac(algorithm, key)`            | `ring`                | Supported            |
| `crypto.getMacs()`                                        | `getMacs()`                             | Native                | Supported            |
| `crypto.argon2(algorithm, parameters, [cb])`              | `argon2(algorithm, parameters)`         | `argon2`              | Supported            |
| `crypto.argon2Sync(algorithm, parameters)`                | `argon2Sync(algorithm, parameters)`     | `argon2`              | Supported            |
| `crypto.pbkdf2(password, salt, iter, keylen, digest)`     | `pbkdf2(...)`                           | `pbkdf2`              | Supported            |
| `crypto.pbkdf2Sync(password, salt, iter, keylen, digest)` | `pbkdf2Sync(...)`                       | `pbkdf2`              | Supported            |
| `crypto.hkdf(digest, ikm, salt, info, keylen)`            | `hkdf(...)`                             | `hkdf`                | Supported            |
| `crypto.hkdfSync(digest, ikm, salt, info, keylen)`        | `hkdfSync(...)`                         | `hkdf`                | Supported            |
| `crypto.getCiphers()`                                     | `getCiphers()`                          | Native                | Supported            |
| `crypto.getCipherInfo(name)`                              | `getCipherInfo(name)`                   | Native                | Supported            |
| `crypto.encryptAead(...)`                                 | `encryptAead(...)`                      | `aes-gcm`             | Supported            |
| `crypto.decryptAead(...)`                                 | `decryptAead(...)`                      | `aes-gcm`             | Supported            |
| `crypto.randomBytes(size)`                                | `randomBytes(size)`                     | `ring::rand`          | Supported            |
| `crypto.randomFillSync(buffer, offset, size)`             | `randomFillSync(...)`                   | `ring::rand`          | Supported            |
| `crypto.randomInt(min, max)`                              | `randomInt(min, max)`                   | `ring::rand`          | Supported            |
| `crypto.randomUUID()`                                     | `randomUuid()`                          | `uuid` (v4)           | Supported            |
| `crypto.randomUUIDv7()`                                   | `randomUuidv7()`                        | `uuid` (v7)           | Supported            |
| `crypto.getRandomValues(buffer)`                          | `getRandomValues(buffer)`               | `ring::rand`          | Supported            |
| `crypto.createECDH(curveName)`                            | `createEcdh(curveName)`                 | `p256`, `p384`        | Supported            |
| `crypto.getCurves()`                                      | `getCurves()`                           | Native                | Supported            |
| `crypto.generateKeyPair(type, options)`                   | `generateKeyPair(...)`                  | `rsa`                 | Supported            |
| `crypto.generateKeyPairSync(type, options)`               | `generateKeyPairSync(...)`              | `rsa`                 | Supported            |
| `crypto.publicEncrypt(key, buffer)`                       | `publicEncrypt(key, buffer)`            | `rsa`                 | Supported            |
| `crypto.privateDecrypt(key, buffer)`                      | `privateDecrypt(key, buffer)`           | `rsa`                 | Supported            |
| `crypto.privateEncrypt(key, buffer)`                      | `privateEncrypt(key, buffer)`           | `rsa`                 | Supported            |
| `crypto.publicDecrypt(key, buffer)`                       | `publicDecrypt(key, buffer)`            | `rsa`                 | Supported            |
| `crypto.createSign(algorithm)`                            | `createSign(algorithm)`                 | `rsa`, `sha2`         | Supported            |
| `crypto.createVerify(algorithm)`                          | `createVerify(algorithm)`               | `rsa`, `sha2`         | Supported            |
| `crypto.sign(algorithm, data, privateKey)`                | `sign(...)`                             | `rsa`, `sha2`         | Supported            |
| `crypto.verify(algorithm, data, publicKey, sig)`          | `verify(...)`                           | `rsa`, `sha2`         | Supported            |
| `crypto.createPublicKey(key)`                             | `createPublicKey(key)`                  | Native                | Supported            |
| `crypto.createPrivateKey(key)`                            | `createPrivateKey(key)`                 | Native                | Supported            |
| `crypto.createSecretKey(key)`                             | `createSecretKey(key)`                  | Native                | Supported            |
| `x25519GenerateKeypair()`                                 | `x25519GenerateKeypair()`               | `x25519-dalek`        | Supported            |
| `x25519DiffieHellman(priv, pub)`                          | `x25519DiffieHellman(priv, pub)`        | `x25519-dalek`        | Supported            |
| `crypto.encapsulate(key)`                                 | `encapsulate(key)`                      | `x25519-dalek`        | Supported            |
| `crypto.decapsulate(key, ciphertext)`                     | `decapsulate(key, ciphertext)`          | `x25519-dalek`        | Supported            |
| `new X509Certificate(buffer)`                             | `X509Certificate` class                 | `x509-parser`         | Supported            |
| `new TLS(provider)`                                       | `TLS` class                             | `rustls`              | Supported            |
| `validatePkcs8PrivateKey(pem)`                            | `validatePkcs8PrivateKey(pem)`          | `rsa`                 | Supported            |
| `validateSpkiPublicKey(pem)`                              | `validateSpkiPublicKey(pem)`            | `rsa`                 | Supported            |
