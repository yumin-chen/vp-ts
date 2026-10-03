# `@lib/crypto` API Specifications & Compatibility Matrix

`@lib/crypto` provides high-performance Node.js compatible cryptographic operations powered by the Rust crate `lib_crypto_native` and `napi-rs`.

---

## 1. Core Modules Overview

- **`crypto_hasher`** (`src/crypto_hasher.rs`): Provides `Hash` class, `createHash`, `hash` one-shot function, and `getHashes`.
- **`argon2`** (`src/argon2.rs`): Provides `argon2Hash`, `argon2HashSync`, `argon2HashRaw`, `argon2HashRawSync`, `argon2Verify`, `argon2VerifySync`, and `argon2ParseOptions` following `argon2-rust`.
- **`hmac`** (`src/hmac.rs`): Provides `Hmac` class and `createHmac` using `ring::hmac`.
- **`pbkdf2`** (`src/pbkdf2.rs`): Provides `pbkdf2` async task and `pbkdf2Sync` using `ring::pbkdf2`.
- **`tls`** (`src/tls.rs`): Provides `TLS` class supporting multiple underlying cryptographic providers: `ring` (default), `openssl` (via `rustls-openssl`), `btls` (BoringSSL via `boring-rustls-provider`), and `mbedtls` (via `rustls-mbedtls-provider`).

---

## 2. NAPI-RS Auto-Generated TypeScript Types

All TypeScript declarations (`index.d.ts`) are auto-generated directly from Rust `#[napi]` declarations using explicit NAPI-RS type attributes (`#[napi(ts_type = "...")]`, `#[napi(ts_args_type = "...")]`, `#[napi(ts_return_type = "...")]`).

---

## 3. API Compatibility Table

| `node:crypto` API                        | `@lib/crypto` Equivalent                                 | Status                 | Backend / Underlying Engine                                                     | Notes                                                             |
| ---------------------------------------- | -------------------------------------------------------- | ---------------------- | ------------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| `crypto.createHmac(algorithm, key)`      | `createHmac`, `new Hmac(...)`                            | **Supported**          | `ring::hmac`                                                                    | Supports SHA1, SHA256, SHA384, SHA512                             |
| `crypto.createHash(algorithm)`           | `createHash`, `new Hash(...)`                            | **Supported**          | `ring::digest` / `rustls`                                                       | Supports SHA1, SHA256, SHA384, SHA512, SHA512-256                 |
| `crypto.hash(algorithm, data, encoding)` | `hash(...)`                                              | **Supported**          | `ring::digest`                                                                  | One-shot hashing utility                                          |
| `crypto.getHashes()`                     | `getHashes()`                                            | **Supported**          | Rust Crate                                                                      | Returns array of supported digest algorithms                      |
| `crypto.pbkdf2(...)`                     | `pbkdf2(...)`                                            | **Supported**          | `ring::pbkdf2`                                                                  | Async PBKDF2 task                                                 |
| `crypto.pbkdf2Sync(...)`                 | `pbkdf2Sync(...)`                                        | **Supported**          | `ring::pbkdf2`                                                                  | Synchronous PBKDF2 derivation                                     |
| Argon2 Hashing                           | `argon2Hash`, `argon2HashSync`, `argon2VerifySync`, etc. | **Supported**          | `argon2-rust`                                                                   | Supports Argon2i, Argon2d, Argon2id and PHC options parsing       |
| TLS Engine                               | `new TLS(provider)`                                      | **Supported**          | `rustls`, `rustls-openssl`, `boring-rustls-provider`, `rustls-mbedtls-provider` | Providers: `ring` (default primary), `openssl`, `btls`, `mbedtls` |
| `crypto.createMac(algorithm, key)`       | `createHmac` / `Hmac`                                    | **Partial**            | `ring::hmac`                                                                    | Delegated to HMAC provider                                        |
| `crypto.createPrivateKey`                | Native `KeyObject`                                       | **Planned / Deferred** | OpenSSL / WebCrypto                                                             | KeyObject parsing                                                 |
| `crypto.createPublicKey`                 | Native `KeyObject`                                       | **Planned / Deferred** | OpenSSL / WebCrypto                                                             | KeyObject parsing                                                 |
| `crypto.createSecretKey`                 | Native `KeyObject`                                       | **Planned / Deferred** | OpenSSL / WebCrypto                                                             | KeyObject parsing                                                 |
| `crypto.encapsulate` / `decapsulate`     | KEM APIs                                                 | **Planned / Deferred** | ML-KEM / Post-Quantum                                                           | KEM algorithms                                                    |
| `crypto.generateKeyPair`                 | KeyPair Generator                                        | **Planned / Deferred** | RSA / EC / Ed25519                                                              | Asymmetric key generation                                         |
| `crypto.randomBytes` / `randomFill`      | PRNG Utilities                                           | **Supported**          | `ring::rand` / WebCrypto                                                        | Secure random bytes generation                                    |
| `crypto.timingSafeEqual`                 | Constant-time Comparison                                 | **Supported**          | Constant-time byte comparison                                                   | Constant-time byte array equality check                           |

---

## 4. Native Export Map (`src/lib.rs`)

```rust
#[path = "crypto_hasher.rs"]
pub(crate) mod crypto_hasher;

#[path = "argon2.rs"]
pub(crate) mod argon2;

#[path = "hmac.rs"]
pub(crate) mod hmac;

#[path = "pbkdf2.rs"]
pub(crate) mod pbkdf2;

#[path = "tls.rs"]
pub(crate) mod tls;

pub use argon2::*;
pub use crypto_hasher::*;
pub use hmac::*;
pub use pbkdf2::*;
pub use tls::*;
```
