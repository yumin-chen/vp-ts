# `@lib/crypto` API Compatibility Specification

This document summarizes the standard `node:crypto` API contracts and the implementation/compatibility status provided by `@lib/crypto` (native Rust binding `lib_crypto_native` via NAPI-RS).

## Overview

Primary defaults use open-source native Rust implementations (`ring`, `rustls`, `argon2`, `rsa`, `p256`, `p384`, `p521`, `x25519-dalek`, `ed25519-dalek`, `uuid`, `hkdf`, `hmac`, `sha2`, `scrypt`, `pkcs8`, `x509-parser`).

---

## Compatibility Table

| `node:crypto` API                                                     | Rust Native Backing / Implementation                             | Status    |
| :-------------------------------------------------------------------- | :--------------------------------------------------------------- | :-------- |
| `crypto.argon2(algorithm, parameters, callback)`                      | `argon2` crate (`argon2d`, `argon2i`, `argon2id`)                | Supported |
| `crypto.argon2Sync(algorithm, parameters)`                            | `argon2` crate                                                   | Supported |
| `crypto.checkPrime(candidate[, options], callback)`                   | `rand` / bit analysis                                            | Supported |
| `crypto.checkPrimeSync(candidate[, options])`                         | `rand` / bit analysis                                            | Supported |
| `crypto.createCipheriv(algorithm, key, iv)`                           | `ring::aead` (AES-128-GCM, AES-256-GCM, ChaCha20-Poly1305, etc.) | Supported |
| `crypto.createDecipheriv(algorithm, key, iv)`                         | `ring::aead`                                                     | Supported |
| `crypto.createDiffieHellman(...)`                                     | `p256` / `ring`                                                  | Supported |
| `crypto.createDiffieHellmanGroup(name)`                               | `p256` / `ring`                                                  | Supported |
| `crypto.createECDH(curveName)`                                        | `p256`, `p384`, `p521`                                           | Supported |
| `crypto.createHash(algorithm)`                                        | `sha2`, `sha1`, `md-5`                                           | Supported |
| `crypto.createHmac(algorithm, key)`                                   | `ring::hmac`                                                     | Supported |
| `crypto.createMac(algorithm, key)`                                    | `ring::hmac`                                                     | Supported |
| `crypto.createPrivateKey(key)`                                        | `KeyObject`                                                      | Supported |
| `crypto.createPublicKey(key)`                                         | `KeyObject`                                                      | Supported |
| `crypto.createSecretKey(key)`                                         | `KeyObject`                                                      | Supported |
| `crypto.createSign(algorithm)`                                        | `rsa`, `sha2`                                                    | Supported |
| `crypto.createVerify(algorithm)`                                      | `rsa`, `sha2`                                                    | Supported |
| `crypto.decapsulate(key, ciphertext)`                                 | KEM abstraction                                                  | Supported |
| `crypto.diffieHellman(options)`                                       | Shared secret computation                                        | Supported |
| `crypto.encapsulate(key)`                                             | KEM abstraction                                                  | Supported |
| `crypto.generateKey(type, options, callback)`                         | `rand`                                                           | Supported |
| `crypto.generateKeySync(type, options)`                               | `rand`                                                           | Supported |
| `crypto.generateKeyPair(type, options, callback)`                     | `rsa`, `pkcs8`                                                   | Supported |
| `crypto.generateKeyPairSync(type, options)`                           | `rsa`, `pkcs8`                                                   | Supported |
| `crypto.generatePrime(size[, options], callback)`                     | `rand`                                                           | Supported |
| `crypto.generatePrimeSync(size[, options])`                           | `rand`                                                           | Supported |
| `crypto.getCipherInfo(nameOrNid)`                                     | Cipher metadata query                                            | Supported |
| `crypto.getCiphers()`                                                 | Ciphers query                                                    | Supported |
| `crypto.getCurves()`                                                  | Elliptic curves query                                            | Supported |
| `crypto.getDiffieHellman(groupName)`                                  | Diffie-Hellman group query                                       | Supported |
| `crypto.getFips()`                                                    | FIPS mode query                                                  | Supported |
| `crypto.getHashes()`                                                  | Hashes query                                                     | Supported |
| `crypto.getMacs()`                                                    | MAC implementations query                                        | Supported |
| `crypto.getRandomValues(typedArray)`                                  | WebCrypto random fill                                            | Supported |
| `crypto.hash(algorithm, data, options)`                               | One-shot hashing                                                 | Supported |
| `crypto.hkdf(digest, ikm, salt, info, keylen, callback)`              | `hkdf` crate                                                     | Supported |
| `crypto.hkdfSync(digest, ikm, salt, info, keylen)`                    | `hkdf` crate                                                     | Supported |
| `crypto.parsePKCS12(bundle)`                                          | `pkcs8`                                                          | Supported |
| `crypto.pbkdf2(password, salt, iterations, keylen, digest, callback)` | `ring::pbkdf2`                                                   | Supported |
| `crypto.pbkdf2Sync(password, salt, iterations, keylen, digest)`       | `ring::pbkdf2`                                                   | Supported |
| `crypto.privateDecrypt(privateKey, buffer)`                           | `rsa`                                                            | Supported |
| `crypto.privateEncrypt(privateKey, buffer)`                           | `rsa`                                                            | Supported |
| `crypto.publicDecrypt(key, buffer)`                                   | `rsa`                                                            | Supported |
| `crypto.publicEncrypt(key, buffer)`                                   | `rsa`                                                            | Supported |
| `crypto.randomBytes(size)`                                            | `rand`                                                           | Supported |
| `crypto.randomFill(buffer, offset, size)`                             | `rand`                                                           | Supported |
| `crypto.randomFillSync(buffer, offset, size)`                         | `rand`                                                           | Supported |
| `crypto.randomInt(min, max)`                                          | `rand`                                                           | Supported |
| `crypto.randomUUID(options)`                                          | `uuid` (v4)                                                      | Supported |
| `crypto.randomUUIDv7(options)`                                        | `uuid` (v7)                                                      | Supported |
| `crypto.scrypt(password, salt, keylen, options, callback)`            | `scrypt` crate                                                   | Supported |
| `crypto.scryptSync(password, salt, keylen, options)`                  | `scrypt` crate                                                   | Supported |
| `crypto.secureHeapUsed()`                                             | Memory statistics                                                | Supported |
| `crypto.setEngine(engine, flags)`                                     | Engine settings                                                  | Supported |
| `crypto.setFips(bool)`                                                | FIPS settings                                                    | Supported |
| `crypto.sign(algorithm, data, key)`                                   | `rsa`, `sha2`                                                    | Supported |
| `crypto.subtle`                                                       | WebCrypto subtle alias                                           | Supported |
| `crypto.timingSafeEqual(a, b)`                                        | Constant-time comparison                                         | Supported |
| `crypto.verify(algorithm, data, key, signature)`                      | `rsa`, `sha2`                                                    | Supported |
| `crypto.webcrypto`                                                    | WebCrypto alias                                                  | Supported |

---

## Classes & Constructors

- `Certificate`
- `Cipher` / `Cipheriv`
- `Decipher` / `Decipheriv`
- `DiffieHellman` / `DiffieHellmanGroup` / `ECDH` / `Ecdh`
- `Hash`
- `Hmac`
- `KeyObject`
- `CryptoKeyPair`
- `Sign`
- `Verify`
- `X509Certificate`
- `TLS` / `Tls`
