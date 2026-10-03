# Node:Crypto API Compatibility Matrix

This document tabulates the Node.js `node:crypto` API standard alongside `@lib/crypto` (`lib_crypto_native`) implementation details and compatibility.

## API Compatibility Summary

| API / Method | `node:crypto` Support | `@lib/crypto` Support | Underlying Engine / Details |
|---|---|---|---|
| `crypto.createHash(algorithm)` | Yes | Supported | NAPI Hasher class (`ring` digest context). Method chaining `.update()` supported. Supports `sha256`, `sha384`, `sha512`, `sha512_256`, `sha1`. |
| `crypto.hash(algorithm, data, encoding)` | Yes | Supported | NAPI one-shot hash function using `ring`. Accepts `String` or `Buffer`. |
| `crypto.getHashes()` | Yes | Supported | Returns available hash digest algorithms (`sha256`, `sha384`, `sha512`, `sha512_256`, `sha1`). |
| `crypto.createHmac(algorithm, key)` | Yes | Supported | NAPI Hmac class (`ring::hmac`). Accepts `String` or `Buffer` key and data. |
| `crypto.Hmac` | Yes | Supported | NAPI Hmac struct (`update`, `digest`). Method chaining `.update()` supported. |
| `crypto.pbkdf2Sync(password, salt, iterations, keylen, digest)` | Yes | Supported | NAPI synchronous PBKDF2 function (`ring::pbkdf2`). |
| `crypto.pbkdf2(password, salt, iterations, keylen, digest, callback)` | Yes | Supported | NAPI async PBKDF2 function (`ring::pbkdf2` offloaded via `AsyncTask` threadpool). |
| `crypto.Pbkdf2` | N/A (custom wrapper) | Supported | Class helper wrapping synchronous and asynchronous PBKDF2 key derivation. |
| `crypto.argon2` / `crypto.argon2Sync` | Yes | Supported | Argon2 key hashing backed by standard Rust `argon2` crate. |
| `crypto.randomBytes` / `crypto.randomFillSync` | Yes | Supported | Cryptographically secure random bytes generated via `ring::rand::SystemRandom`. |
| `crypto.randomInt` / `crypto.randomUUID` | Yes | Supported | Uniform random integer and RFC 4122 v4 UUID generator. |
| `crypto.createSecretKey` / `createPublicKey` / `createPrivateKey` | Yes | Supported | NAPI `KeyObject` representation (`secret`, `public`, `private`). |
| `crypto.X509Certificate` | Yes | Supported | NAPI X509 Certificate representation. |
| `crypto.createSign` / `crypto.sign` | Yes | Supported | `Sign` streaming class and one-shot signing functions. |
| `crypto.createVerify` / `crypto.verify` / `crypto.createMac` | Yes | Supported | `Verify` streaming class and verification/MAC helpers. |
| `crypto.aeadEncrypt` | Yes | Supported | AEAD ciphers (`aes-128-gcm`, `aes-256-gcm`, `chacha20-poly1305`) via `ring::aead`. |
| `crypto.createECDH` / `crypto.ECDH` | Yes | Supported | Elliptic Curve Diffie-Hellman key exchange helper. |
| `crypto.generateKeyPairSync` | Yes | Supported | RSA key pair generator helper. |
| `crypto.TLS` | N/A (custom wrapper) | Supported | NAPI TLS class providing provider selection: OpenSSL (`rustls-openssl`), BTLS / BoringSSL (`boring-rustls-provider`), MbedTLS (`rustls-mbedtls-provider-utils`), defaulting to `ring`. |
| `crypto.getDefaultProviderName()` | N/A (custom wrapper) | Supported | Returns default provider (`ring`). |
| `crypto.getSupportedProviders()` | N/A (custom wrapper) | Supported | Returns list of supported TLS providers (`["ring", "openssl", "btls", "mbedtls"]`). |
