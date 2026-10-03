# `@lib/crypto` (`lib_crypto_native`)

Node:crypto compatible native cryptographic library built with NAPI-RS and backed by pure Rust cryptographic crates (`ring`, `rustls`, `argon2`, etc.).

## Features

- **Hasher & HMAC**: Stream & one-shot digests supporting SHA-256, SHA-384, SHA-512, SHA-1 with method chaining.
- **PBKDF2 & Argon2**: High-performance key derivation functions with async offloading via NAPI worker threads.
- **AEAD & ECDH & RSA**: AEAD ciphers (`aes-gcm`, `chacha20-poly1305`), Elliptic Curve Diffie-Hellman, and RSA key pair generation.
- **Sign & Verify**: Streaming and one-shot cryptographic signing and verification.
- **Random & Utilities**: Secure random bytes (`randomBytes`), random integer (`randomInt`), UUID v4 (`randomUuid`), `KeyObject`, and `X509Certificate`.
- **TLS Provider Selector**: Flexible TLS provider engine selection supporting `ring` (default), `openssl` (`rustls-openssl`), `btls` (`boring-rustls-provider`), and `mbedtls`.

## Cross-Building & Target Platforms

### Target Matrix

Configured targets in `package.json` (`napi.targets`):
- `x86_64-apple-darwin` / `aarch64-apple-darwin` (macOS x64 / ARM64)
- `x86_64-pc-windows-msvc` / `i686-pc-windows-msvc` / `aarch64-pc-windows-msvc` (Windows x64 / x86 / ARM64)
- `x86_64-unknown-linux-gnu` / `aarch64-unknown-linux-gnu` / `armv7-unknown-linux-gnueabihf` (Linux glibc x64 / ARM64 / ARMv7)
- `x86_64-unknown-linux-musl` / `aarch64-unknown-linux-musl` (Linux musl x64 / ARM64)
- `aarch64-linux-android` / `armv7-linux-androideabi` (Android ARM64 / ARMv7)
- `x86_64-unknown-freebsd` (FreeBSD x64)
- `wasm32-wasip1-threads` (WASM / WASI)

### Do we need to commit `npm/*` folders?

**No.** Committing `npm/<platform-arch-abi>` directories to git is **no longer recommended** by NAPI-RS.

Instead, create them dynamically in CI during the release pipeline:
```bash
# Generate platform package directories dynamically in CI
napi create-npm-dirs
```

In the release pipeline:
1. `napi build --platform --release` builds binary artifacts per target platform job.
2. `napi create-npm-dirs` creates `npm/<target>` directories containing target-specific `package.json` files.
3. `napi artifacts` copies built `.node` binaries into their corresponding platform directories.
4. `napi pre-publish -t npm` publishes the root package and all optional platform packages to npm.

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
