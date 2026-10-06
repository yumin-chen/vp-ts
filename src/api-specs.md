# `@lib/crypto` API Compatibility Specification

This document summarizes the `node:crypto` compatible module implementation in Rust using `napi-rs`.

## Standard `node:crypto` APIs & Compatibility Matrix

| API Name           | Standard Node.js Signature / Class                             | Compatibility Status | Underlying Engine / Library                | Notes                                                                   |
| ------------------ | -------------------------------------------------------------- | -------------------- | ------------------------------------------ | ----------------------------------------------------------------------- |
| `argon2`           | `argon2(algorithm, parameters, callback)`                      | Implemented          | `argon2-rust`                              | Supports Argon2d, Argon2i, Argon2id, secret & AD                        |
| `argon2Sync`       | `argon2Sync(algorithm, parameters)`                            | Implemented          | `argon2-rust`                              | Synchronous Argon2 hashing                                              |
| `randomBytes`      | `randomBytes(size[, callback])`                                | Implemented          | `rand::rngs::OsRng`                        | Generates cryptographically secure random bytes                         |
| `randomFill`       | `randomFill(buffer[, offset][, size], callback)`               | Implemented          | `rand::rngs::OsRng`                        | Fills buffer with random bytes asynchronously                           |
| `randomFillSync`   | `randomFillSync(buffer[, offset][, size])`                     | Implemented          | `rand::rngs::OsRng`                        | Fills buffer with random bytes synchronously                            |
| `randomInt`        | `randomInt([min, ]max[, callback])`                            | Implemented          | `rand::Rng::gen_range`                     | Generates uniform random integer without modulo bias                    |
| `randomUUID`       | `randomUUID([options])`                                        | Implemented          | `rand::rngs::OsRng`                        | RFC 4122 v4 UUID generator                                              |
| `randomUUIDv7`     | `randomUUIDv7([options])`                                      | Implemented          | `rand::rngs::OsRng`                        | RFC 9562 v7 UUID generator with timestamp                               |
| `createHmac`       | `createHmac(algorithm, key)`                                   | Implemented          | `ring::hmac`                               | Supports SHA1, SHA256, SHA384, SHA512                                   |
| `Hmac`             | `class Hmac`                                                   | Implemented          | `ring::hmac`                               | `update(data)`, `digest(encoding)`                                      |
| `pbkdf2`           | `pbkdf2(password, salt, iterations, keylen, digest, callback)` | Implemented          | `ring::pbkdf2`                             | Asynchronous PBKDF2 offloaded to threadpool                             |
| `pbkdf2Sync`       | `pbkdf2Sync(password, salt, iterations, keylen, digest)`       | Implemented          | `ring::pbkdf2`                             | Synchronous PBKDF2                                                      |
| `aeadEncrypt`      | `aeadEncrypt(algorithm, key, iv, plaintext, aad)`              | Implemented          | `aes-gcm`, `aes-gcm-siv`, `aes-siv`, `ccm` | AEAD encryption for GCM, GCM-SIV, SIV, CCM                              |
| `aeadDecrypt`      | `aeadDecrypt(algorithm, key, iv, ciphertext, tag, aad)`        | Implemented          | `aes-gcm`, `aes-gcm-siv`, `aes-siv`, `ccm` | AEAD decryption for GCM, GCM-SIV, SIV, CCM                              |
| `ECDH`             | `class ECDH`                                                   | Implemented          | `p256`, `x25519-dalek`                     | Diffie-Hellman key agreement for P-256 & X25519                         |
| `KeyObject`        | `class KeyObject`                                              | Implemented          | Native Rust struct                         | Secret, Public, Private key representation with export, equals, details |
| `createSecretKey`  | `createSecretKey(key)`                                         | Implemented          | Native Rust                                | Creates secret KeyObject                                                |
| `createPublicKey`  | `createPublicKey(key)`                                         | Implemented          | Native Rust                                | Creates public KeyObject                                                |
| `createPrivateKey` | `createPrivateKey(key)`                                        | Implemented          | Native Rust                                | Creates private KeyObject                                               |
| `CryptoKeyPair`    | `class CryptoKeyPair`                                          | Implemented          | Native Rust struct                         | Pair of `publicKey` and `privateKey`                                    |
| `Sign`             | `class Sign`                                                   | Implemented          | `ed25519-dalek`                            | Message signing (`update`, `sign`)                                      |
| `Verify`           | `class Verify`                                                 | Implemented          | `ed25519-dalek`                            | Signature verification (`update`, `verify`)                             |
| `X509Certificate`  | `class X509Certificate`                                        | Implemented          | `x509-parser`                              | Certificate parsing, fingerprints (1/256/512), subject, issuer, serial  |
| `TLS`              | `class TLS`                                                    | Implemented          | `rustls`                                   | Multi-provider architecture (`ring`, `openssl`, `boringssl`, `mbedtls`) |
| `CryptoHasher`     | `class CryptoHasher`                                           | Implemented          | `ring::digest`                             | Streaming message hashing (`update`, `digest`)                          |
| `hash`             | `hash(algorithm, data, outputEncoding)`                        | Implemented          | `ring::digest`                             | One-shot hashing                                                        |
| `getHashes`        | `getHashes()`                                                  | Implemented          | Native                                     | Returns supported digest algorithm names                                |
| `publicEncrypt`    | `publicEncrypt(key, buffer)`                                   | Implemented          | `rsa` crate                                | RSA public key encryption                                               |
| `privateDecrypt`   | `privateDecrypt(privateKey, buffer)`                           | Implemented          | `rsa` crate                                | RSA private key decryption                                              |
| `publicDecrypt`    | `publicDecrypt(key, buffer)`                                   | Implemented          | `rsa` crate                                | RSA public key decryption                                               |
| `privateEncrypt`   | `privateEncrypt(privateKey, buffer)`                           | Implemented          | `rsa` crate                                | RSA private key encryption                                              |
