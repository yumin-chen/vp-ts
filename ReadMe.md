# `@lib/crypto` (`lib_crypto_native`)

Node.js `node:crypto` compatible cryptographic native module built using NAPI-RS and Rust cryptographic libraries.

## Package Naming Convention

- **npm Package Scope**: `@lib/crypto`
- **Cargo Crate Name**: `lib_crypto_native` (following the `[scope]_` convention where `@lib/` maps to `lib_`)

## Architecture & `no-std` / POSIX Strategy

`@lib/crypto` prioritizes open-source native Rust implementations (`ring`, `rustls`, `argon2-rust`, `aes-gcm`, `p256`, `rsa`, `uuid`) as primary defaults.

### `no-std` / POSIX Compatibility without `libstd`
- Core cryptographic algorithms and data structures rely on `core` and `alloc` abstractions rather than requiring standard OS runtime dependencies.
- Optional TLS provider backends (such as OpenSSL, BoringSSL, MbedTLS) are dynamically feature-flagged or safely fall back to the native `ring` / `rustls` provider if an external system library is missing or cannot be initialized.

## Supported `node:crypto` Modules & APIs

- **HMAC**: `Hmac` class, `createHmac(algorithm, key)`
- **CryptoHasher**: `CryptoHasher` class, `createHash(algorithm)`, `cryptoHash(algorithm, data, encoding)`
- **PBKDF2**: `pbkdf2` (async NAPI task), `pbkdf2Sync(password, salt, iterations, keylen, digest)`
- **Argon2**: `argon2Hash`, `argon2HashSync`, `argon2Verify`, `argon2VerifySync`, `argon2ParseOptions`, `argon2Sync`
- **TLS Provider**: `TLS` class, `createTls(provider)` with support for `Ring`, `OpenSSL`, `BoringSSL`, `MbedTLS` with graceful fallback
- **Random Generators**: `randomBytes`, `randomFillSync`, `randomInt`, `randomUUID`, `randomUUIDv7`
- **AEAD**: `encryptAesGcm`, `decryptAesGcm`
- **ECDH & Diffie-Hellman**: `ECDH` class, `createECDH`, `DiffieHellman` class, `createDiffieHellman`, `createDiffieHellmanGroup`
- **RSA**: `generateKeyPairSync`, `publicEncrypt`, `privateDecrypt`, `privateEncrypt`, `publicDecrypt`
- **Key Object & Certificates**: `KeyObject`, `CryptoKeyPair`, `X509Certificate`
- **Signatures & Verification**: `Sign` class, `createSign`, `sign`, `Verify` class, `createVerify`
- **Key Agreement**: `createPublicKey`, `createSecretKey`, `createMac`, `encapsulate`, `decapsulate`

## Building & Testing

```bash
# Build native binary and auto-generate TypeScript definitions to ./dist
npm run build

# Run unit test suite
npm test
npx vp test src/*.test.ts
```
