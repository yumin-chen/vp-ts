# `@lib/crypto` API Compatibility Specifications

| Module / API                                                | Native Provider                      | Status      | Notes                                                        |
| :---------------------------------------------------------- | :----------------------------------- | :---------- | :----------------------------------------------------------- |
| `argon2` / `argon2Sync`                                     | `argon2-rust`                        | Implemented | Argon2id password hashing                                    |
| `createHmac` / `Hmac`                                       | `ring`                               | Implemented | SHA1, SHA256, SHA384, SHA512                                 |
| `pbkdf2` / `pbkdf2Sync`                                     | `pbkdf2` crate                       | Implemented | SHA256, SHA512 key derivation                                |
| `encryptAead` / `decryptAead`                               | `aes-gcm`                            | Implemented | AES-128-GCM, AES-256-GCM                                     |
| `createECDH` / `Ecdh`                                       | `p256`                               | Implemented | Prime256v1 / P-256 curve support                             |
| `X25519DiffieHellman`                                       | `x25519-dalek`                       | Implemented | X25519 ECDH key agreement                                    |
| `generateKeyPair` / `generateKeyPairSync`                   | `rsa`                                | Implemented | RSA key pair generation & PKCS#8 PEM exports                 |
| `randomBytes` / `randomInt` / `randomUUID` / `randomUUIDv7` | `rand` / `uuid`                      | Implemented | Cryptographic PRNG & UUID v4/v7                              |
| `KeyObject` / `CryptoKeyPair` / `X509Certificate`           | Native Rust                          | Implemented | Key objects and X509 cert wrappers                           |
| `Sign` / `Verify`                                           | `p256::ecdsa`                        | Implemented | ECDSA P-256 signing and verification                         |
| `Tls`                                                       | `rustls` / `ring` / `rustls-openssl` | Implemented | TLS abstraction supporting Ring, OpenSSL, BoringSSL, MbedTLS |
| `createHash` / `Hash` / `getHashes`                         | `ring`                               | Implemented | Hash digest creation                                         |
