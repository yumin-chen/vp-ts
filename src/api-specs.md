# `@lib/crypto` API Compatibility Specification

This document summarizes the standard `node:crypto` API contracts and the implementation/compatibility status provided by `@lib/crypto` (native Rust binding `lib_crypto_native` via NAPI-RS).

## Overview

Primary defaults use open-source native Rust implementations (`ring`, `rustls`, `argon2`, `rsa`, `p256`, `p384`, `p521`, `x25519-dalek`, `ed25519-dalek`, `uuid`, `hkdf`, `hmac`, `sha2`, `pkcs8`, `x509-parser`).

---

## Compatibility Table

| `node:crypto` API                                                     | Rust Native Backing / Implementation                             | Status    |
| :-------------------------------------------------------------------- | :--------------------------------------------------------------- | :-------- |
| `crypto.argon2(algorithm, parameters, callback)`                      | `argon2` crate (`argon2d`, `argon2i`, `argon2id`)                | Supported |
| `crypto.argon2Sync(algorithm, parameters)`                            | `argon2` crate                                                   | Supported |
| `crypto.createHmac(algorithm, key)`                                   | `ring::hmac`                                                     | Supported |
| `Class: Hmac`                                                         | `ring::hmac`                                                     | Supported |
| `crypto.createMac(algorithm, key)`                                    | `ring::hmac`                                                     | Supported |
| `crypto.pbkdf2(password, salt, iterations, keylen, digest, callback)` | `ring::pbkdf2`                                                   | Supported |
| `crypto.pbkdf2Sync(password, salt, iterations, keylen, digest)`       | `ring::pbkdf2`                                                   | Supported |
| `crypto.createCipheriv(algorithm, key, iv)`                           | `ring::aead` (AES-128-GCM, AES-256-GCM, ChaCha20-Poly1305, etc.) | Supported |
| `crypto.createDecipheriv(algorithm, key, iv)`                         | `ring::aead`                                                     | Supported |
| `Class: Cipher` / `Class: Cipheriv`                                   | `ring::aead`                                                     | Supported |
| `Class: Decipher` / `Class: Decipheriv`                               | `ring::aead`                                                     | Supported |
| `crypto.createECDH(curveName)`                                        | `p256`, `p384`, `p521`                                           | Supported |
| `crypto.createDiffieHellman(...)`                                     | `p256` / `ring`                                                  | Supported |
| `crypto.createDiffieHellmanGroup(name)`                               | `p256` / `ring`                                                  | Supported |
| `Class: ECDH`                                                         | `p256` / `ring`                                                  | Supported |
| `crypto.generateKeyPair(type, options, callback)`                     | `rsa`, `pkcs8`                                                   | Supported |
| `crypto.generateKeyPairSync(type, options)`                           | `rsa`, `pkcs8`                                                   | Supported |
| `crypto.publicEncrypt(key, buffer)`                                   | `rsa`                                                            | Supported |
| `crypto.privateDecrypt(privateKey, buffer)`                           | `rsa`                                                            | Supported |
| `crypto.randomBytes(size)`                                            | `rand`                                                           | Supported |
| `crypto.randomFill(buffer, offset, size)`                             | `rand`                                                           | Supported |
| `crypto.randomFillSync(buffer, offset, size)`                         | `rand`                                                           | Supported |
| `crypto.randomInt(min, max)`                                          | `rand`                                                           | Supported |
| `crypto.randomUUID(options)`                                          | `uuid` (v4)                                                      | Supported |
| `crypto.randomUUIDv7(options)`                                        | `uuid` (v7)                                                      | Supported |
| `crypto.createPublicKey(key)`                                         | `KeyObject`                                                      | Supported |
| `crypto.createPrivateKey(key)`                                        | `KeyObject`                                                      | Supported |
| `crypto.createSecretKey(key)`                                         | `KeyObject`                                                      | Supported |
| `Class: KeyObject`                                                    | `lib_crypto_native`                                              | Supported |
| `Class: CryptoKeyPair`                                                | `lib_crypto_native`                                              | Supported |
| `Class: X509Certificate`                                              | `x509-parser`                                                    | Supported |
| `crypto.encapsulate(key)`                                             | KEM abstraction                                                  | Supported |
| `crypto.decapsulate(key, ciphertext)`                                 | KEM abstraction                                                  | Supported |
| `crypto.diffieHellman(options)`                                       | Shared secret computation                                        | Supported |
| `crypto.sign(algorithm, data, key)`                                   | `rsa`, `sha2`                                                    | Supported |
| `crypto.verify(algorithm, data, key, signature)`                      | `rsa`, `sha2`                                                    | Supported |
| `crypto.createSign(algorithm)`                                        | `rsa`, `sha2`                                                    | Supported |
| `crypto.createVerify(algorithm)`                                      | `rsa`, `sha2`                                                    | Supported |
| `Class: Sign`                                                         | `rsa`, `sha2`                                                    | Supported |
| `Class: Verify`                                                       | `rsa`, `sha2`                                                    | Supported |
| `crypto.createHash(algorithm)`                                        | `sha2`, `sha1`, `md-5`, `digest`                                 | Supported |
| `crypto.hash(algorithm, data, encoding)`                              | `sha2`, `sha1`, `md-5`                                           | Supported |
| `crypto.getHashes()`                                                  | `sha2`, `sha1`, `md-5`                                           | Supported |
| `crypto.hkdf(digest, ikm, salt, info, keylen)`                        | `hkdf`                                                           | Supported |
| `crypto.hkdfSync(digest, ikm, salt, info, keylen)`                    | `hkdf`                                                           | Supported |
| `crypto.parsePKCS12(bundle)` / PKCS#8 helpers                         | `pkcs8`                                                          | Supported |
| `Class: TLS`                                                          | `rustls`                                                         | Supported |

---

## Argon2 Specification Contract (`node:crypto` v24.7.0)

- `argon2Sync(algorithm, parameters)`:
  - `algorithm`: `'argon2id'`, `'argon2i'`, or `'argon2d'`
  - `parameters`: `{ message, nonce, parallelism, tagLength, memory, passes, secret?, associatedData? }`
  - Returns: `<Buffer>`
