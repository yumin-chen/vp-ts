# `@lib/crypto` (`lib_crypto_native`) API Compatibility Specifications

This document tabulates the standard Node.js `node:crypto` API specification against our `@lib/crypto` native implementation.

---

## 1. Classes & Instance Methods

| `node:crypto` Class / Method | `@lib/crypto` Binding Signature | Compatibility Status | Implementation Notes / Backend |
| :--- | :--- | :--- | :--- |
| **`Class: Certificate`** | N/A | **Planned** | SPKAC certificate utilities |
| **`Class: Cipheriv`** | `encryptAesGcm` / `decryptAesGcm` | **Partial** | Direct AEAD functions exported via `aead.rs` using `aes-gcm` |
| **`Class: Decipheriv`** | `decryptAesGcm` | **Partial** | Direct AEAD functions exported via `aead.rs` using `aes-gcm` |
| **`Class: DiffieHellman`** | `new DiffieHellman(primeLengthOrBytes)` / `createDiffieHellman` | **Full** | MODP modular exponentiation using `num-bigint` |
| `diffieHellman.computeSecret(otherPublicKey)` | `dh.computeSecret(otherPublicKey)` | **Full** | Calculates `g^x mod p` shared secret |
| `diffieHellman.generateKeys()` | `dh.generateKeys()` | **Full** | Generates DH key pair |
| `diffieHellman.getGenerator()` | `dh.getGenerator()` | **Full** | Returns generator buffer |
| `diffieHellman.getPrime()` | `dh.getPrime()` | **Full** | Returns prime buffer |
| `diffieHellman.getPrivateKey()` | `dh.getPrivateKey()` | **Full** | Returns private key buffer |
| `diffieHellman.getPublicKey()` | `dh.getPublicKey()` | **Full** | Returns public key buffer |
| **`Class: DiffieHellmanGroup`** | `createDiffieHellmanGroup(name)` | **Full** | Supports MODP groups (modp14, modp16, etc.) |
| **`Class: ECDH`** | `new ECDH(curveName)` / `createECDH(curveName)` | **Full** | Elliptic curve Diffie-Hellman (`p256`) |
| `ecdh.computeSecret(otherPublicKey)` | `ecdh.computeSecret(otherPublicKey)` | **Full** | Real `p256::ecdh::diffie_hellman` shared secret computation |
| `ecdh.generateKeys()` | `ecdh.generateKeys()` | **Full** | Generates EC key pair |
| `ecdh.getPrivateKey()` | `ecdh.getPrivateKey()` | **Full** | Returns EC private key |
| `ecdh.getPublicKey()` | `ecdh.getPublicKey()` | **Full** | Returns EC public key |
| **`Class: Hash`** / **`CryptoHasher`** | `new CryptoHasher(algorithm)` / `createHash(algorithm)` | **Full** | `rustls` / `ring` digest provider |
| `hash.update(data[, inputEncoding])` | `hasher.update(data, encoding)` | **Full** | Incremental hash update |
| `hash.digest([encoding])` | `hasher.digest(encoding)` | **Full** | Produces hex/base64/buffer digest |
| **`Class: Hmac`** | `new Hmac(algorithm, key)` / `createHmac(algorithm, key)` | **Full** | `ring::hmac` implementation |
| `hmac.update(data[, inputEncoding])` | `hmac.update(data, encoding)` | **Full** | Incremental HMAC update |
| `hmac.digest([encoding])` | `hmac.digest(encoding)` | **Full** | Produces HMAC digest |
| **`Class: KeyObject`** | `new KeyObject(type, data)` | **Full** | In-memory key representation |
| `keyObject.type` | `keyObject.type` | **Full** | 'secret' \| 'public' \| 'private' |
| `keyObject.asymmetricKeyType` | `keyObject.asymmetricKeyType` | **Full** | Key algorithm type |
| `keyObject.export([options])` | `keyObject.export()` | **Full** | Exports key material buffer |
| **`Class: Mac`** | `createMac(algorithm, key)` | **Full** | Provider MAC implementation |
| **`Class: Sign`** | `new Sign(algorithm)` / `createSign(algorithm)` | **Full** | Signature generator using `rsa` & `ed25519-dalek` |
| `sign.update(data[, inputEncoding])` | `sign.update(data)` | **Full** | Incremental signature update |
| `sign.sign(privateKey[, outputEncoding])` | `sign.sign(privateKey, encoding)` | **Full** | Digital signing using private key |
| **`Class: Verify`** | `new Verify(algorithm)` / `createVerify(algorithm)` | **Full** | Signature verifier using `rsa` & `ed25519-dalek` |
| `verify.update(data[, inputEncoding])` | `verify.update(data)` | **Full** | Incremental verify update |
| `verify.verify(key, signature[, signatureEncoding])` | `verify.verify(key, signature)` | **Full** | Signature verification |
| **`Class: X509Certificate`** | `new X509Certificate(buffer)` | **Full** | Real certificate parsing via `x509-parser` |
| `x509.checkEmail(email[, options])` | `x509.checkEmail(email)` | **Full** | Matches email address |
| `x509.checkHost(name[, options])` | `x509.checkHost(name)` | **Full** | Matches hostname |
| `x509.checkIP(ip)` | `x509.checkIP(ip)` | **Full** | Matches IP address |
| `x509.fingerprint` / `256` / `512` | `x509.fingerprint` / `256` / `512` | **Full** | Certificate fingerprints |
| `x509.issuer` / `x509.subject` | `x509.issuer` / `x509.subject` | **Full** | Parsed certificate DN strings |
| `x509.raw` / `x509.serialNumber` | `x509.raw` / `x509.serialNumber` | **Full** | DER buffer and serial number |
| `x509.validFrom` / `x509.validTo` | `x509.validFrom` / `x509.validTo` | **Full** | Validity date strings |
| **`Class: TLS`** (Extension) | `new TLS(provider)` / `createTls(provider)` | **Full Extension** | Multi-backend TLS provider (`Ring`, `OpenSSL`, `BoringSSL`, `MbedTLS`) |

---

## 2. Module Methods & Properties

| `node:crypto` Function | `@lib/crypto` Binding Signature | Compatibility Status | Implementation Notes / Backend |
| :--- | :--- | :--- | :--- |
| `crypto.argon2(algorithm, parameters, callback)` | `argon2(algorithm, parameters)` | **Full** | Async Argon2 key derivation |
| `crypto.argon2Sync(algorithm, parameters)` | `argon2Sync(algorithm, parameters)` | **Full** | Synchronous Argon2 key derivation |
| `crypto.createDiffieHellman(...)` | `createDiffieHellman(...)` | **Full** | Diffie-Hellman object creation |
| `crypto.createDiffieHellmanGroup(name)` | `createDiffieHellmanGroup(name)` | **Full** | MODP group DH object creation |
| `crypto.createECDH(curveName)` | `createECDH(curveName)` | **Full** | ECDH object creation |
| `crypto.createHash(algorithm[, options])` | `createHash(algorithm)` | **Full** | Hash object creation |
| `crypto.createHmac(algorithm, key[, options])` | `createHmac(algorithm, key)` | **Full** | HMAC object creation |
| `crypto.createMac(algorithm, key[, options])` | `createMac(algorithm, key)` | **Full** | MAC object creation |
| `crypto.createPublicKey(key)` | `createPublicKey(key)` | **Full** | Public key object import |
| `crypto.createSecretKey(key[, encoding])` | `createSecretKey(key)` | **Full** | Secret key object import |
| `crypto.createSign(algorithm[, options])` | `createSign(algorithm)` | **Full** | Sign object creation |
| `crypto.createVerify(algorithm[, options])` | `createVerify(algorithm)` | **Full** | Verify object creation |
| `crypto.decapsulate(key, ciphertext[, callback])` | `decapsulate(key, ciphertext)` | **Full** | KEM key decapsulation |
| `crypto.encapsulate(key[, callback])` | `encapsulate(key)` | **Full** | KEM key encapsulation |
| `crypto.generateKeyPairSync(type, options)` | `generateKeyPairSync(type, options)` | **Full** | Synchronous RSA key pair generation (`rsa` crate) |
| `crypto.getCiphers()` | `getCiphers()` | **Full** | Returns list of supported cipher names |
| `crypto.getCurves()` | `getCurves()` | **Full** | Returns list of supported elliptic curves |
| `crypto.getHashes()` | `getHashes()` | **Full** | Returns supported digest algorithms |
| `crypto.getMacs()` | `getMacs()` | **Full** | Returns supported MAC algorithms |
| `crypto.hash(algorithm, data[, options])` | `cryptoHash(algorithm, data, encoding)` | **Full** | One-shot hash digest |
| `crypto.pbkdf2(...)` / `crypto.pbkdf2Sync(...)` | `pbkdf2` / `pbkdf2Sync` | **Full** | PBKDF2 derivation using `ring::pbkdf2` |
| `crypto.privateDecrypt(...)` / `crypto.privateEncrypt(...)` | `privateDecrypt` / `privateEncrypt` | **Full** | Real RSA private key decryption/encryption (`rsa` crate) |
| `crypto.publicDecrypt(...)` / `crypto.publicEncrypt(...)` | `publicDecrypt` / `publicEncrypt` | **Full** | Real RSA public key decryption/encryption (`rsa` crate) |
| `crypto.randomBytes(size[, callback])` | `randomBytes(size)` | **Full** | CSPRNG random bytes using `ring::rand` |
| `crypto.randomFillSync(buffer[, offset][, size])` | `randomFillSync(...)` | **Full** | Synchronous random buffer fill |
| `crypto.randomInt([min, ]max[, callback])` | `randomInt(min, max)` | **Full** | Unbiased random integer generator |
| `crypto.randomUUID([options])` | `randomUUID()` | **Full** | RFC 4122 version 4 UUID generator (`uuid` crate) |
| `crypto.randomUUIDv7([options])` | `randomUUIDv7()` | **Full** | RFC 9562 version 7 UUID generator (`uuid` crate) |
| `crypto.sign(algorithm, data, key[, callback])` | `sign(algorithm, data, key)` | **Full** | One-shot digital signing |
| `crypto.timingSafeEqual(a, b)` | `timingSafeEqual(a, b)` | **Full** | Constant-time byte array comparison |
