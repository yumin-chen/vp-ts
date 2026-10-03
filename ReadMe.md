# `@lib/crypto` (`lib_crypto_native`)

Node:crypto compatible native module implemented in Rust using NAPI-RS.

- **NPM package name:** `@lib/crypto`
- **Cargo crate name:** `lib_crypto_native` (following the `[scope]_` convention: `@lib/` -> `lib_crypto_native`)

## Support for `no-std` & POSIX without `libstd`

To support environments where Rust's standard library (`std`) is unavailable (such as bare-metal, embedded POSIX environments, or minimal WASI/POSIX targets without `libstd`), crate dependencies and code structure can be conditionally feature-gated:

1. **Cargo Feature Flags (`std` vs `no_std` + `alloc`):**
   - The crate defines `default = ["std"]`.
   - Core cryptographic primitives (`ring`, `rsa`, `p256`, `uuid`, `x509-parser`, `argon2-rust`) support `no_std` when used with Rust's `alloc` crate.
   - When `std` feature is disabled (`--no-default-features`), memory allocations use `extern crate alloc;` and POSIX system entropy interfaces or OS syscalls (`getrandom` / `dev_urandom`) are linked directly without relying on `std::*` OS APIs.

2. **NAPI Bindings Abstraction:**
   - On Node.js NAPI platforms, `std` is active by default to bind smoothly with libuv threadpools and Node runtime.
   - For minimal embedded POSIX builds, native bindings stub or feature-flag heavy providers, falling back to core `ring` / `rustls` primitives.

## Modules Implemented

- **`crypto_hasher`**: `Hash` class, `createHash`, `hash`, `getHashes` using `ring`.
- **`hmac`**: `Hmac` class, `createHmac` using `ring`.
- **`pbkdf2`**: `pbkdf2`, `pbkdf2Sync` using `ring`.
- **`argon2`**: `argon2Hash`, `argon2Verify`, `argon2ParseOptions`, `argon2`, `argon2Sync` using `argon2-rust`.
- **`rand`**: `randomBytes`, `randomFill`, `randomFillSync`, `randomInt`, `randomUUID`.
- **`aead`**: AEAD cipher implementations (AES-GCM, CCM, SIV).
- **`ecdh`**: `ECDH` class, `createECDH`, `createDiffieHellman`, `createDiffieHellmanGroup`.
- **`rsa`**: `generateKeyPair`, `generateKeyPairSync`.
- **`key_object`**: `KeyObject`, `CryptoKeyPair`, `X509Certificate` classes.
- **`agreement`**: Key agreement (`crypto.encapsulate`, `crypto.decapsulate`), `Verify` class, X.509 certificate inspector methods.
- **`signature`**: `crypto.sign`, `crypto.createSign`, `Sign` class.
- **`tls`**: `TLS` class providing Rustls crypto providers (Ring as default, OpenSSL, BoringSSL, MbedTLS).

## Building & Testing

```bash
# Build native NAPI addon
npm run build:napi

# Run tests
npm test
```
