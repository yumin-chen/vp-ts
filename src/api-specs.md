# `@lib/crypto` API Compatibility Specifications

This document tabulates the standard Node.js `node:crypto` APIs and our implementation and compatibility status in `@lib/crypto` (built using NAPI-RS and Rust cryptographic libraries like `ring`, `rustls`, `argon2-rust`, `rustls-openssl`, `boring-rustls-provider`, and `rustls-mbedtls-provider`).

## API Compatibility Status Table

| Standard `node:crypto` API | `@lib/crypto` Implementation | Status | Notes / Backend |
| :--- | :--- | :--- | :--- |
| `crypto.createHmac(algorithm, key[, options])` | `createHmac(algorithm, key)` / `new Hmac(algorithm, key)` | **Full** | Implemented using `ring::hmac`. Supports SHA-1, SHA-256, SHA-384, SHA-512. Chaining and digest encodings (`hex`, `base64`, `latin1`, `buffer`) supported. |
| `crypto.createHash(algorithm[, options])` / `crypto.hash(...)` | `createHash(algorithm)` / `new CryptoHasher(algorithm)` / `cryptoHash(algorithm, data, encoding)` | **Full** | Implemented using `rustls`/`ring` digest provider. Supports SHA-1, SHA-256, SHA-384, SHA-512. |
| `crypto.pbkdf2(password, salt, iterations, keylen, digest, callback)` | `pbkdf2(password, salt, iterations, keylen, digest)` | **Full** | Async task via NAPI using `ring::pbkdf2`. Returns Promise resolving to Buffer. |
| `crypto.pbkdf2Sync(password, salt, iterations, keylen, digest)` | `pbkdf2Sync(password, salt, iterations, keylen, digest)` | **Full** | Synchronous derivation using `ring::pbkdf2`. Returns Buffer. |
| Argon2 Hashing (`hash`, `hashSync`, `verify`, `verifySync`, etc.) | `argon2Hash`, `argon2HashSync`, `argon2Verify`, `argon2VerifySync`, `argon2HashRaw`, `argon2HashRawSync`, `argon2ParseOptions` | **Full Extension** | Implemented using `argon2-rust` crate with support for Argon2d, Argon2i, Argon2id algorithms and PHC encoded string parsing. |
| TLS Crypto Providers (`class TLS`) | `new Tls(provider)` / `createTls(provider)` | **Full Extension** | Provides multi-backend TLS crypto provider support via `rustls`: Ring (default), OpenSSL (`rustls-openssl`), BoringSSL (`boring-rustls-provider`), and MbedTLS (`rustls-mbedtls-provider`). |
| `crypto.createMac(algorithm, key[, options])` | `createHmac` / `createMac` | **Partial** | Provider MAC algorithms mapped to HMAC. |
| `crypto.createPrivateKey(key)` | N/A | **Planned** | KeyObject management handled via provider keys. |
| `crypto.createPublicKey(key)` | N/A | **Planned** | Public key parsing and extraction. |
| `crypto.createSecretKey(key[, encoding])` | N/A | **Planned** | Secret key object wrapper. |
| `crypto.createSign(algorithm[, options])` | N/A | **Planned** | Digital signature generation. |
| `crypto.createVerify(algorithm[, options])` | N/A | **Planned** | Digital signature verification. |
| `crypto.decapsulate(...)` / `crypto.encapsulate(...)` | N/A | **Planned** | KEM operations (ML-KEM, DHKEM). |
| `crypto.diffieHellman(...)` | N/A | **Planned** | Diffie-Hellman key exchange. |
| `crypto.generateKey(...)` / `crypto.generateKeySync(...)` | N/A | **Planned** | Secret key generation. |
| `crypto.generateKeyPair(...)` / `crypto.generateKeyPairSync(...)` | N/A | **Planned** | Asymmetric key pair generation. |
| `crypto.generatePrime(...)` / `crypto.generatePrimeSync(...)` | N/A | **Planned** | Prime number generation. |
| `crypto.getCipherInfo(nameOrNid)` / `crypto.getCiphers()` | N/A | **Planned** | Cipher metadata. |
| `crypto.getCurves()` | N/A | **Planned** | Elliptic curve list. |
| `crypto.getDiffieHellman(groupName)` | N/A | **Planned** | Predefined DH groups. |
| `crypto.getFips()` / `crypto.setFips(bool)` | N/A | **Planned** | FIPS state query/toggle. |
| `crypto.getHashes()` | N/A | **Full** | Lists supported digest algorithms (`sha1`, `sha256`, `sha384`, `sha512`). |
| `crypto.getMacs()` | N/A | **Full** | Lists supported MAC algorithms (`hmac`). |
| `crypto.getRandomValues(typedArray)` | N/A | **Full** | Cryptographic pseudorandom byte filling using `ring::rand`. |
| `crypto.hkdf(...)` / `crypto.hkdfSync(...)` | N/A | **Planned** | HKDF key derivation function. |
| `crypto.parsePKCS12(...)` | N/A | **Planned** | PKCS#12 bundle parser. |
| `crypto.privateDecrypt(...)` / `crypto.privateEncrypt(...)` | N/A | **Planned** | RSA private key encryption/decryption. |
| `crypto.publicDecrypt(...)` / `crypto.publicEncrypt(...)` | N/A | **Planned** | RSA public key encryption/decryption. |
| `crypto.randomBytes(...)` / `crypto.randomFillSync(...)` | N/A | **Full** | CSPRNG random bytes. |
| `crypto.randomInt([min, ]max)` | N/A | **Full** | Unbiased random integer generation. |
| `crypto.randomUUID()` / `crypto.randomUUIDv7()` | N/A | **Full** | RFC 4122 v4 and RFC 9562 v7 UUID generation. |
| `crypto.scrypt(...)` / `crypto.scryptSync(...)` | N/A | **Planned** | Scrypt password-based key derivation. |
| `crypto.secureHeapUsed()` | N/A | **Planned** | Secure heap usage statistics. |
| `crypto.sign(...)` / `crypto.verify(...)` | N/A | **Planned** | One-shot digital signing and verification. |
| `crypto.timingSafeEqual(a, b)` | N/A | **Full** | Constant-time byte array comparison using `ring::constant_time`. |
| `crypto.webcrypto` / `crypto.subtle` | N/A | **Planned** | Web Crypto standard API bindings. |
