# `@lib/crypto` API Compatibility Specification

This document tabulates the standard `node:crypto` API coverage and compatibility status implemented natively in Rust via NAPI-RS.

---

## 📊 API Compatibility Table

| `node:crypto` API Category | API Function / Class                                       | Rust Implementation (`src/*.rs`) | Status       | Notes                                           |
| :------------------------- | :--------------------------------------------------------- | :------------------------------- | :----------- | :---------------------------------------------- |
| **Hasher / Digest**        | `crypto.createHash`, `CryptoHasher`                        | `src/crypto_hasher.rs`           | ✅ Supported | Implemented via `sha2` crate (SHA-256, SHA-512) |
| **HMAC**                   | `crypto.createHmac`, `HmacHasher`                          | `src/hmac.rs`                    | ✅ Supported | Implemented via `hmac` and `sha2` crates        |
| **PBKDF2**                 | `crypto.pbkdf2Sync`                                        | `src/pbkdf2.rs`                  | ✅ Supported | Implemented via `pbkdf2` crate                  |
| **Argon2**                 | `crypto.argon2Sync`, `crypto.argon2VerifySync`             | `src/argon2.rs`                  | ✅ Supported | Implemented via `argon2` crate                  |
| **AEAD / Ciphers**         | `encryptGcm`, `decryptGcm`                                 | `src/aead.rs`                    | ✅ Supported | Implemented via `aes-gcm` crate                 |
| **ECDH / DH**              | `crypto.createECDH`, `Ecdh`                                | `src/ecdh.rs`                    | ✅ Supported | Implemented via `ring::agreement` (X25519)      |
| **RSA KeyGen**             | `crypto.generateKeyPairSync`                               | `src/rsa.rs`                     | ✅ Supported | Implemented via `rsa` crate                     |
| **Random Utilities**       | `randomBytes`, `randomFillSync`, `randomInt`, `randomUUID` | `src/rand.rs`                    | ✅ Supported | Implemented via `rand` and `uuid` crates        |
| **Key Objects**            | `KeyObject`, `createSecretKey`                             | `src/key_object.rs`              | ✅ Supported | Native NAPI class wrapping secret key bytes     |
| **Signature**              | `createSign`, `Signer`                                     | `src/signature.rs`               | ✅ Supported | Implemented via `sha2` digest signing           |
| **Key Agreement**          | `diffieHellmanSecret`                                      | `src/agreement.rs`               | ✅ Supported | Shared secret computation                       |
| **TLS Provider**           | `TLS` class                                                | `src/tls.rs`                     | ✅ Supported | Primary default provider: `ring`                |

---

## 🛠️ Primary Defaults & Cryptographic Providers

1. **Ring & Rustls First**: Native open-source Rust cryptography (`ring`, `rustls`) is used as the primary default provider.
2. **Type Declarations**: TypeScript declarations (`index.d.ts`) are auto-generated directly from `#[napi]` Rust attributes without manual TS declaration editing.
