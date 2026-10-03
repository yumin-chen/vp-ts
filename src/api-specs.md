# Node:Crypto API Specifications & Compatibility Status

This document tabulates the standard Node.js `node:crypto` API and our `@lib/crypto` implementation and compatibility status.

| Standard `node:crypto` API | @lib/crypto API | Status / Notes |
| --- | --- | --- |
| `crypto.createHash(algorithm, options)` | `createHash(algorithm)` / `Hash` class | **Supported** - Implemented via `ring` digest. Supports SHA-1, SHA-256, SHA-384, SHA-512, SHA-512/256. |
| `crypto.hash(algorithm, data, outputEncoding)` | `hash(algorithm, data, outputEncoding)` | **Supported** - One-shot hash digest helper function. |
| `crypto.getHashes()` | `getHashes()` | **Supported** - Returns list of supported hash algorithms (`sha1`, `sha256`, `sha384`, `sha512`, `sha512_256`). |
| `crypto.createHmac(algorithm, key, options)` | `createHmac(algorithm, key)` / `Hmac` class | **Supported** - Implemented via `ring` HMAC. |
| `crypto.pbkdf2(password, salt, iterations, keylen, digest, callback)` | `pbkdf2(password, salt, iterations, keylen, digest)` | **Supported** - Async PBKDF2 returning a `Promise<Buffer>`. |
| `crypto.pbkdf2Sync(password, salt, iterations, keylen, digest)` | `pbkdf2Sync(password, salt, iterations, keylen, digest)` | **Supported** - Synchronous PBKDF2 returning `Buffer`. |
| `crypto.argon2` (custom/extension) | `argon2Hash`, `argon2HashSync`, `argon2HashRaw`, `argon2HashRawSync`, `argon2Verify`, `argon2VerifySync`, `argon2ParseOptions` | **Supported** - Implemented via `argon2-rust` supporting Argon2i, Argon2d, Argon2id, custom parameters and PHC strings. |
| `TLS` / `Tls` | `TLS` class / `TlsProvider` | **Supported** - Provides provider choices for Rustls: Ring (default), OpenSSL (`rustls-openssl`), BoringSSL (`boring-rustls-provider`), MbedTLS (`TlsProvider.Mbedtls`). |
| `crypto.randomBytes(size)` | Not in this scope | Planned for future extensions. |
| `crypto.randomUUID()` | Not in this scope | Planned for future extensions. |
| `crypto.generateKeyPair` / `crypto.sign` / `crypto.verify` | Not in this scope | Planned for future extensions. |
| `crypto.hkdf` / `crypto.scrypt` | Not in this scope | Planned for future extensions. |
