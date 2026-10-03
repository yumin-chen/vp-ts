# `@lib/crypto` API Compatibility Specification

This document summarizes the `node:crypto` compatible module implementation in Rust using `napi-rs`.

## Standard `node:crypto` APIs & Compatibility Matrix

| API Name          | Standard Node.js Signature / Class                       | Compatibility Status | Underlying Rust Provider / Library | Notes                                                                                |
| ----------------- | -------------------------------------------------------- | -------------------- | ---------------------------------- | ------------------------------------------------------------------------------------ |
| `argon2`          | `argon2(password, salt, options)`                        | Implemented          | `argon2` crate                     | Supports Argon2id, custom memory/time cost, parallelism, secret & AD                 |
| `argon2Sync`      | `argon2Sync(password, salt, options)`                    | Implemented          | `argon2` crate                     | Synchronous Argon2 hashing                                                           |
| `createHmac`      | `createHmac(algorithm, key)`                             | Implemented          | `ring::hmac`                       | Supports SHA1, SHA256, SHA384, SHA512                                                |
| `Hmac`            | `class Hmac`                                             | Implemented          | `ring::hmac`                       | `update(data)`, `digest(encoding)`                                                   |
| `pbkdf2`          | `pbkdf2(password, salt, iterations, keylen, digest)`     | Implemented          | `ring::pbkdf2` / `pbkdf2`          | Asynchronous key derivation                                                          |
| `pbkdf2Sync`      | `pbkdf2Sync(password, salt, iterations, keylen, digest)` | Implemented          | `ring::pbkdf2` / `pbkdf2`          | Synchronous key derivation                                                           |
| `aeadEncrypt`     | `aeadEncrypt(algorithm, key, iv, plaintext, aad)`        | Implemented          | `aes-gcm`, `aes-siv`, `ccm`        | AEAD encryption for GCM, SIV, CCM modes                                              |
| `aeadDecrypt`     | `aeadDecrypt(algorithm, key, iv, ciphertext, tag, aad)`  | Implemented          | `aes-gcm`, `aes-siv`, `ccm`        | AEAD decryption for GCM, SIV, CCM modes                                              |
| `ECDH`            | `class ECDH`                                             | Implemented          | `p256`, `x25519-dalek`             | Diffie-Hellman key agreement for P-256 & X25519                                      |
| `KeyObject`       | `class KeyObject`                                        | Implemented          | Native Rust struct                 | Secret, Public, Private key representation                                           |
| `CryptoKeyPair`   | `class CryptoKeyPair`                                    | Implemented          | Native Rust struct                 | Pair of `publicKey` and `privateKey`                                                 |
| `Sign`            | `class Sign`                                             | Implemented          | `ed25519-dalek`                    | Message signing (`update`, `sign`)                                                   |
| `Verify`          | `class Verify`                                           | Implemented          | `ed25519-dalek`                    | Signature verification (`update`, `verify`)                                          |
| `X509Certificate` | `class X509Certificate`                                  | Implemented          | `x509-parser`                      | Certificate parsing, fingerprints (1/256/512), subject, issuer, serial, verification |
| `TLS`             | `class TLS`                                              | Implemented          | `rustls` (defaults to `ring`)      | Multi-provider architecture (`ring`, `openssl`, `boringssl`, `mbedtls`)              |
| `CryptoHasher`    | `class CryptoHasher`                                     | Implemented          | `ring::digest`                     | Streaming message hashing (`update`, `digest`)                                       |
| `hash`            | `hash(algorithm, data, outputEncoding)`                  | Implemented          | `ring::digest`                     | One-shot hashing                                                                     |
| `getHashes`       | `getHashes()`                                            | Implemented          | Native                             | Returns supported digest algorithm names                                             |

---

## Detailed Module Specs

### 1. `argon2` & `argon2Sync`

- **Engine**: `argon2` Rust crate.
- **Algorithms**: Argon2id (default, v0x13).
- **Options**: `memoryCost`, `timeCost`, `parallelism`, `tagLength`, `secret`, `associatedData`.

### 2. `Hmac`

- **Engine**: `ring::hmac`.
- **Supported Digests**: `sha1`, `sha256`, `sha384`, `sha512`.
- **Output Encodings**: `hex`, `base64`, `Buffer` (raw bytes).

### 3. `PBKDF2`

- **Engine**: `ring::pbkdf2`.
- **Supported Digests**: `sha1`, `sha256`, `sha384`, `sha512`.

### 4. `AEAD`

- **Engine**: Rust Crypto (`aes-gcm`, `aes-siv`, `ccm`).
- **Modes Supported**:
  - `aes-128-gcm`, `aes-256-gcm`
  - `aes-128-siv`, `aes-256-siv`

### 5. `ECDH`, `KeyObject`, `CryptoKeyPair`, `Sign`, `Verify`, `X509Certificate`

- **ECDH**: P-256 and X25519 support.
- **X509Certificate**:
  - Subject, Issuer, Serial Number, Validity range (from/to).
  - Fingerprints: SHA1 (`fingerprint`), SHA256 (`fingerprint256`), SHA512 (`fingerprint512`).
  - Validation: `checkIssued`, `verify`.

### 6. `TLS`

- **Engine**: `rustls` infrastructure.
- **Providers**: `ring` (default), `openssl` (via `rustls-openssl`), `boringssl` (via `boring-rustls-provider`), `mbedtls` (via `rustls-mbedtls-provider`).
