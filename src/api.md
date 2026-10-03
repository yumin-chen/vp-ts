# Node:Crypto API Compatibility Matrix

This document tabulates the Node.js `node:crypto` API standard alongside `@lib/crypto` implementation details and compatibility.

## API Compatibility Summary

| API / Method | `node:crypto` Support | `@lib/crypto` Support | Underlying Engine / Details |
|---|---|---|---|
| `crypto.createHash(algorithm)` | Yes | Supported | NAPI Hasher class (`ring` digest context). Supports `sha256`, `sha384`, `sha512`, `sha512_256`, `sha1`. |
| `crypto.hash(algorithm, data, encoding)` | Yes | Supported | NAPI one-shot hash function using `ring`. |
| `crypto.getHashes()` | Yes | Supported | Returns available hash digest algorithms (`sha256`, `sha384`, `sha512`, `sha512_256`, `sha1`). |
| `crypto.createHmac(algorithm, key)` | Yes | Supported | NAPI Hmac class (`ring::hmac`). Supports `sha256`, `sha384`, `sha512`, `sha1`. |
| `crypto.Hmac` | Yes | Supported | NAPI Hmac struct (`update`, `digest`). |
| `crypto.pbkdf2Sync(password, salt, iterations, keylen, digest)` | Yes | Supported | NAPI synchronous PBKDF2 function (`ring::pbkdf2`). |
| `crypto.pbkdf2(password, salt, iterations, keylen, digest, callback)` | Yes | Supported | NAPI async PBKDF2 function (`ring::pbkdf2` offloaded via `AsyncTask` threadpool). Returns Promise/AsyncTask. |
| `crypto.Pbkdf2` | N/A (custom wrapper) | Supported | Class helper wrapping synchronous and asynchronous PBKDF2 key derivation. |
| `crypto.TLS` | N/A (custom wrapper) | Supported | NAPI TLS class providing provider selection: OpenSSL (`rustls-openssl`), BTLS / BoringSSL (`boring-rustls-provider`), MbedTLS (`rustls-mbedtls-provider-utils`), defaulting to `ring`. |
| `crypto.getDefaultProviderName()` | N/A (custom wrapper) | Supported | Returns default provider (`ring`). |
| `crypto.getSupportedProviders()` | N/A (custom wrapper) | Supported | Returns list of supported TLS providers (`["ring", "openssl", "btls", "mbedtls"]`). |
| `crypto.createCipheriv` / `crypto.createDecipheriv` | Yes | Not Implemented | Planned for future release. |
| `crypto.randomBytes` | Yes | Not Implemented | Native WebCrypto or system random bytes planned for future release. |
| `crypto.generateKeyPair` / `crypto.generateKeyPairSync` | Yes | Not Implemented | Planned for future release. |
| `crypto.sign` / `crypto.verify` | Yes | Not Implemented | Planned for future release. |
| `crypto.hkdf` / `crypto.hkdfSync` | Yes | Not Implemented | Planned for future release. |
| `crypto.scrypt` / `crypto.scryptSync` | Yes | Not Implemented | Planned for future release. |

## Feature Implementation Details

### Hasher (`crypto_hasher.rs`)
- Implements `Hasher` class, `createHash`, `hash`, and `getHashes`.
- Uses `ring::digest` algorithms.
- Supports output encodings: `'hex'`, `'base64'`, `'buffer'`.

### HMAC (`hmac.rs`)
- Implements `Hmac` class and `createHmac`.
- Uses `ring::hmac`.
- Supports methods `update(buffer)` and `digest(encoding)`.

### PBKDF2 (`pbkdf2.rs`)
- Implements `pbkdf2Sync`, `pbkdf2` (async), and `Pbkdf2` class.
- Uses `ring::pbkdf2` with `PBKDF2_HMAC_SHA256`, `PBKDF2_HMAC_SHA384`, `PBKDF2_HMAC_SHA512`, and `PBKDF2_HMAC_SHA1`.
- Asynchronous computation is executed on NAPI worker threads to prevent blocking Node.js event loop.

### TLS Provider (`tls.rs` & `rustls.rs`)
- Implements `Tls` class and provider getters.
- Integrates Rust TLS providers:
  - Default: `ring` (`rustls::crypto::ring`)
  - OpenSSL: `rustls-openssl`
  - BoringSSL (BTLS): `boring-rustls-provider`
  - MbedTLS: `rustls-mbedtls-provider-utils`
