# `@lib/crypto` (`lib_crypto_native`)

Node:crypto compatible native cryptographic library built with NAPI-RS and backed by pure Rust cryptographic crates (`ring`, `rustls`, `argon2`, etc.).

## Features

- **Hasher & HMAC**: Stream & one-shot digests supporting SHA-256, SHA-384, SHA-512, SHA-1 with method chaining.
- **PBKDF2 & Argon2**: High-performance key derivation functions with async offloading via NAPI worker threads.
- **AEAD & ECDH & RSA**: AEAD ciphers (`aes-gcm`, `chacha20-poly1305`), Elliptic Curve Diffie-Hellman, and RSA key pair generation.
- **Sign & Verify**: Streaming and one-shot cryptographic signing and verification.
- **Random & Utilities**: Secure random bytes (`randomBytes`), random integer (`randomInt`), UUID v4 (`randomUuid`), `KeyObject`, and `X509Certificate`.
- **TLS Provider Selector**: Flexible TLS provider engine selection supporting `ring` (default), `openssl` (`rustls-openssl`), `btls` (`boring-rustls-provider`), and `mbedtls`.

## `no-std` / POSIX Portability Architecture

To support embedded, POSIX-only, or `no-std` target environments where `libstd` is absent:
1. **Core Cryptographic Primitives**: `ring` and `argon2` support `#![no_std]` mode when compiled with `default-features = false` and relying only on the `alloc` crate for dynamic memory allocation (`extern crate alloc;`).
2. **System Entropy**: Randomness source falls back to POSIX `getrandom` syscall / `/dev/urandom` directly or custom entropy hooks without requiring full OS abstractions from `std`.
3. **NAPI / C CType ABI Boundary**: NAPI-RS provides C-ABI `extern "C"` bindings over raw pointers and lengths (`*const u8`, `usize`), allowing modular integration in bare-metal, WASM/WASI, or POSIX targets.

## Development & Usage

### Build
```bash
npm run build
```

### Test
```bash
npm test
```
