# Node:Crypto API Specifications & Compatibility Status

This document tabulates the standard `node:crypto` APIs alongside the implementation details and compatibility status of `@lib/crypto` (built using NAPI-RS and Rust crate `lib_crypto_native`).

## Package & Cargo Naming Convention

- **npm package name**: `@lib/crypto`
- **Cargo crate name**: `lib_crypto_native` (following the `[scope]_` prefix rule for `@lib/` scope)

## Core Architecture & NAPI Annotations

- **HMAC & PBKDF2**: Backed by `ring` (`ring::hmac` and `ring::pbkdf2`).
- **Hashing & HKDF**: Backed by `ring::digest`, `ring::hkdf`, and `ring::rand`.
- **TLS**: Implemented via NAPI-RS exposing `TLS` class with support for selectable providers:
  - Primary Default: `ring` (`rustls::crypto::ring::default_provider`)
  - OpenSSL: `openssl` (`rustls-openssl`)
  - BoringSSL: `btls` / `boringssl` (`boring-rustls-provider`)
  - MbedTLS: `mbedtls` (`rustls-mbedcrypto-provider`)
  - Fallback Mechanism: If an unknown/unsupported provider is passed to `new TLS(...)`, it cleanly falls back to `ring` with `isFallback === true`.
- **Type Declarations**: All TypeScript declarations in `index.d.ts` are auto-generated directly from Rust `#[napi]` attributes (`#[napi(ts_args_type = "...")]`, `#[napi(ts_return_type = "...")]`, `#[napi(js_name = "...")]`) without manual `.d.ts` edits.

---

## API Compatibility Table

| Standard Node:crypto API                                        | Method / Function                                | Implementation in `@lib/crypto`         | Compatibility Status | Underlying Provider / Notes                                                             |
| :-------------------------------------------------------------- | :----------------------------------------------- | :-------------------------------------- | :------------------- | :-------------------------------------------------------------------------------------- |
| `crypto.createHmac(algorithm, key)`                             | Factory function returning `Hmac` instance       | `createHmac(algorithm, key)`            | **Implemented**      | Uses `ring::hmac`. Supports SHA-1, SHA-256, SHA-384, SHA-512.                           |
| `crypto.Hmac`                                                   | Class with `update(data)` and `digest(encoding)` | `Hmac` class                            | **Implemented**      | Supports chaining & digest encodings (`hex`, `base64`, `utf8`, `latin1`, `buffer`).     |
| `crypto.createMac(algorithm, key[, options])`                   | OpenSSL MAC provider factory                     | `getMacs()` lists supported MACs        | **Partial**          | `createHmac` supported for HMAC; MAC options interface documented for future expansion. |
| `crypto.pbkdf2Sync(password, salt, iterations, keylen, digest)` | Synchronous PBKDF2 key derivation                | `pbkdf2Sync(...)`                       | **Implemented**      | Uses `ring::pbkdf2`. Supports SHA-1, SHA-256, SHA-384, SHA-512.                         |
| `crypto.pbkdf2(...)` / `PBKDF2`                                 | Async/Class PBKDF2                               | `PBKDF2` class with `deriveSync`        | **Implemented**      | Native binding with `PBKDF2` class wrapper.                                             |
| `TLS`                                                           | Class for multi-provider TLS                     | `TLS` class                             | **Implemented**      | Multi-backend support (`ring` default, `openssl`, `btls`, `mbedtls`).                   |
| `crypto.createHash(algorithm)`                                  | Factory function returning `Hash` instance       | `createHash(algorithm)`                 | **Implemented**      | Uses `ring::digest`. Supports SHA-1, SHA-256, SHA-384, SHA-512, SHA-512/256.            |
| `crypto.Hash`                                                   | Class with `update(data)` and `digest(encoding)` | `Hash` class                            | **Implemented**      | Supports string / buffer inputs and hex / base64 / utf8 / buffer encodings.             |
| `crypto.hash(algorithm, data[, options])`                       | One-shot hashing utility                         | `hash(algorithm, data, outputEncoding)` | **Implemented**      | Fast one-shot hashing function.                                                         |
| `crypto.randomBytes(size[, callback])`                          | Generates cryptographically secure random bytes  | `randomBytes(size)`                     | **Implemented**      | Uses `ring::rand::SystemRandom`.                                                        |
| `crypto.randomFillSync(buffer[, offset][, size])`               | Fills buffer with random bytes synchronously     | `randomFillSync(buffer, offset, size)`  | **Implemented**      | Uses `ring::rand::SystemRandom`.                                                        |
| `crypto.randomUUID([options])`                                  | Generates RFC 4122 v4 UUID string                | `randomUuid()`                          | **Implemented**      | Uses `ring::rand::SystemRandom` formatted to RFC 4122 v4 standard.                      |
| `crypto.timingSafeEqual(a, b)`                                  | Constant-time byte comparison                    | `timingSafeEqual(a, b)`                 | **Implemented**      | Prevents timing attacks on secret byte comparisons.                                     |
| `crypto.hkdfSync(digest, ikm, salt, info, keylen)`              | Synchronous HKDF key derivation (RFC 5869)       | `hkdfSync(...)`                         | **Implemented**      | Uses `ring::hkdf`. Supports SHA-256, SHA-384, SHA-512.                                  |
| `crypto.getHashes()`                                            | Returns array of supported hash algorithms       | `getHashes()`                           | **Implemented**      | Returns available hash algorithms.                                                      |
| `crypto.getCiphers()`                                           | Returns array of supported cipher algorithms     | `getCiphers()`                          | **Implemented**      | Returns supported cipher names.                                                         |
| `crypto.getCurves()`                                            | Returns array of supported elliptic curves       | `getCurves()`                           | **Implemented**      | Returns supported curve names.                                                          |
| `crypto.getMacs()`                                              | Returns array of fetchable MAC implementations   | `getMacs()`                             | **Implemented**      | Returns supported MAC names.                                                            |
| `crypto.createPrivateKey(key)`                                  | Creates private key `KeyObject`                  | -                                       | **Deferred**         | Asymmetric key parsing deferred to PKI module.                                          |
| `crypto.createPublicKey(key)`                                   | Creates public key `KeyObject`                   | -                                       | **Deferred**         | Asymmetric key parsing deferred to PKI module.                                          |
| `crypto.createSecretKey(key)`                                   | Creates secret key `KeyObject`                   | -                                       | **Deferred**         | Symmetric key object wrapper.                                                           |
| `crypto.createSign(algorithm)`                                  | Creates `Sign` stream                            | -                                       | **Deferred**         | Streaming signature calculation.                                                        |
| `crypto.createVerify(algorithm)`                                | Creates `Verify` stream                          | -                                       | **Deferred**         | Streaming signature verification.                                                       |
| `crypto.generateKey(type, options)`                             | Asynchronously generates secret key              | -                                       | **Deferred**         | Random secret key generation.                                                           |
| `crypto.generateKeyPair(type, options)`                         | Asynchronously generates asymmetric key pair     | -                                       | **Deferred**         | Asymmetric key pair generation (RSA, EC, Ed25519).                                      |
| `crypto.generatePrime(size[, options])`                         | Generates pseudorandom prime                     | -                                       | **Deferred**         | Prime generation utilities.                                                             |
| `crypto.getCipherInfo(nameOrNid)`                               | Returns cipher metadata                          | -                                       | **Deferred**         | Cipher metadata querying.                                                               |
| `crypto.getDiffieHellman(groupName)`                            | Returns predefined DH group                      | -                                       | **Deferred**         | Predefined DH group key exchange.                                                       |
| `crypto.getFips()` / `setFips(bool)`                            | Query and toggle FIPS mode                       | -                                       | **Deferred**         | FIPS mode configuration.                                                                |
| `crypto.privateDecrypt` / `privateEncrypt`                      | RSA private key encryption/decryption            | -                                       | **Deferred**         | Asymmetric RSA operations.                                                              |
| `crypto.publicDecrypt` / `publicEncrypt`                        | RSA public key encryption/decryption             | -                                       | **Deferred**         | Asymmetric RSA operations.                                                              |
| `crypto.scryptSync(password, salt, keylen)`                     | Password-based key derivation using Scrypt       | -                                       | **Deferred**         | Scrypt KDF.                                                                             |
| `crypto.sign` / `crypto.verify`                                 | One-shot sign and verify                         | -                                       | **Deferred**         | Digital signatures using private/public keys.                                           |
| `crypto.webcrypto`                                              | Web Crypto API standard implementation           | -                                       | **Deferred**         | Web Crypto standard adapter.                                                            |

---

## Detailed Specification Notes

### HMAC (`src/hmac.rs`)

- Class: `Hmac`
- Factory: `createHmac(algorithm: string, key: string | Buffer): Hmac`
- Methods:
  - `update(data: string | Buffer, encoding?: string): void`
  - `digest(encoding?: string): string | Buffer`
- Algorithms supported: `sha1`, `sha256`, `sha384`, `sha512`.

### PBKDF2 (`src/pbkdf2.rs`)

- Class: `PBKDF2`
- Function: `pbkdf2Sync(password: string | Buffer, salt: string | Buffer, iterations: number, keylen: number, digest: string): Buffer`
- Methods:
  - `new PBKDF2(digest: string, iterations: number)`
  - `deriveSync(password: string | Buffer, salt: string | Buffer, keylen: number): Buffer`
- Algorithms supported: `sha1`, `sha256`, `sha384`, `sha512`.

### TLS (`src/rustls.rs` & `src/tls.rs`)

- Class: `TLS`
- Methods:
  - `new TLS(provider?: string)` (default: `"ring"`)
  - `getProviderName(): string` / getter `providerName`
  - `getAvailableProviders(): Array<string>` (returns `["ring", "openssl", "btls", "mbedtls"]`)
  - `getCipherSuites(): Array<string>`
  - `isCipherSupported(cipher: string): boolean`
  - `isFallback`: boolean getter indicating if fallback to primary `ring` provider occurred.

### Crypto Hasher & Utilities (`src/crypto_hasher.rs`)

- Class: `Hash`
- Functions: `createHash(algorithm)`, `hash(algorithm, data, outputEncoding)`
- Utility Functions: `randomBytes`, `randomFillSync`, `randomUuid`, `timingSafeEqual`, `hkdfSync`, `getHashes`, `getCiphers`, `getCurves`, `getMacs`.
