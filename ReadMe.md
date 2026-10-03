# `@lib/crypto` (`lib_crypto_native`)

Node.js `node:crypto` compatible native module built with NAPI-RS and high-performance Rust cryptographic backends.

## Features & Submodules

- **`crypto_hasher`**: `Hash` class, `createHash`, `hash` one-shot utility, and `getHashes` powered by `ring` & `rustls`.
- **`argon2`**: Argon2 password hashing (`argon2Hash`, `argon2HashSync`, `argon2Verify`, `argon2VerifySync`, `argon2ParseOptions`) powered by `argon2-rust`.
- **`hmac`**: `Hmac` class and `createHmac` powered by `ring::hmac`.
- **`pbkdf2`**: `pbkdf2` async task and `pbkdf2Sync` powered by `ring::pbkdf2`.
- **`tls`**: `TLS` class offering provider choices (`ring` default primary, `openssl`, `btls` / BoringSSL, `mbedtls`).

---

## `no-std` & POSIX Compilation

To support `no-std` or POSIX target environments without requiring Rust's standard library (`libstd`):

1. **Use `core` + `alloc`**:
   Replace `std` dependencies with `core` for language primitives and `alloc` for heap allocations (`Vec`, `String`, `Box`):

   ```rust
   #![no_std]
   extern crate alloc;
   use alloc::string::String;
   use alloc::vec::Vec;
   ```

2. **POSIX C Runtime & `libc`**:
   Rely on `libc` or system POSIX C ABIs (`malloc`, `free`, pthreads) provided by Node.js C-NAPI host environment.

3. **Panic Configuration**:
   In `Cargo.toml`, set `panic = "abort"` to eliminate unwinding overhead and `std::panic` runtime hooks:
   ```toml
   [profile.release]
   panic = "abort"
   ```

---

## Usage & Development

### Installation & Build

```bash
npm install
npm run build
```

### Running Tests

```bash
npm test
```

### Code Check & Formatting

```bash
npm run check
```
