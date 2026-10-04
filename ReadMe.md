# `@lib/crypto`

Native `node:crypto` compatible cryptographic module built with NAPI-RS and powered by Rust (`ring`, `rustls`, `rustls-openssl`, `boring-rustls-provider`, `rustls-mbedcrypto-provider`, `rsa`, `x509-parser`, and `argon2-rust`).

## Modules & Features

- **Hasher (`crypto_hasher.rs`)**: `createHash(algorithm)`, `hash(algorithm, data, encoding)`, `getHashes()`
- **HMAC (`hmac.rs`)**: `createHmac(algorithm, key)`, `Hmac` class supporting `update` and `digest` (`hex`, `base64`, `latin1`, Buffer)
- **PBKDF2 (`pbkdf2.rs`)**: `pbkdf2` (async NAPI task) and `pbkdf2Sync`
- **Argon2 (`argon2.rs`)**: High-performance Argon2 password hashing, verification, PHC option parsing, and Node.js compatible `argon2` / `argon2Sync` functions
- **TLS Engine (`tls.rs`)**: `TLS` class exposing Ring, OpenSSL (`rustls-openssl`), BoringSSL (`boring-rustls-provider`), and MbedTLS (`rustls-mbedcrypto-provider`) backends for `rustls`
- **AEAD (`aead.rs`)**: `encryptAead`, `decryptAead` for AES-128-GCM, AES-256-GCM, and ChaCha20-Poly1305
- **ECDH & DH (`ecdh.rs`)**: `ECDH` class, `createECDH`, `createDiffieHellman`, `createDiffieHellmanGroup`
- **RSA (`rsa.rs`)**: Key pair generation (`generateKeyPair`, `generateKeyPairSync`), `publicEncrypt`, `privateDecrypt`, `privateEncrypt`, `publicDecrypt`
- **Random (`rand.rs`)**: `randomBytes`, `randomFillSync`, `randomInt`, `randomUUID`
- **Sign & Verify (`signature.rs`, `agreement.rs`)**: `Sign` and `Verify` classes, `createSign`, `sign`, `createVerify`, `verify`, `createMac`
- **Key Object & X509 (`key_object.rs`)**: `KeyObject`, `CryptoKeyPair`, `X509Certificate`, `createSecretKey`, `createPublicKey`, `createPrivateKey`

## Usage Examples

### Hash

```ts
import { createHash, hash, getHashes } from '@lib/crypto'

const hasher = createHash('sha256')
hasher.update('Hello, world!')
const digestHex = hasher.digest('hex')

const digestBase64 = hash('sha512', 'Hello, world!', 'base64')
console.log(getHashes())
```

### HMAC

```ts
import { createHmac } from '@lib/crypto'

const hmac = createHmac('sha256', 'my-secret-key')
hmac.update('Message to authenticate')
const macHex = hmac.digest('hex')
```

### PBKDF2

```ts
import { pbkdf2Sync, pbkdf2 } from '@lib/crypto'

const derivedSync = pbkdf2Sync('password', 'salt', 10000, 32, 'sha256')
const derivedAsync = await pbkdf2('password', 'salt', 10000, 32, 'sha512')
```

### Argon2

```ts
import { argon2Sync, argon2HashSync, argon2VerifySync, Algorithm, Version } from '@lib/crypto'

// Node.js raw buffer output
const rawBuffer = argon2Sync('argon2id', { message: 'password', nonce: 'saltsalt' })

// PHC formatted string output
const hashStr = argon2HashSync('my-password', {
  algorithm: Algorithm.Argon2id,
  version: Version.V0x13,
  timeCost: 2,
  memoryCost: 19456,
})
const isValid = argon2VerifySync(hashStr, 'my-password')
```

### Random Utilities

```ts
import { randomBytes, randomInt, randomUUID } from '@lib/crypto'

const bytes = randomBytes(16)
const randNum = randomInt(1, 100)
const uuid = randomUUID()
```

### Sign & Verify

```ts
import { sign, verify, generateKeyPairSync } from '@lib/crypto'

const pair = generateKeyPairSync('ed25519')
const signature = sign('ed25519', 'Data to sign', pair.privateKey)
const isValid = verify('ed25519', 'Data to sign', pair.publicKey, signature)
```

### TLS Engine

```ts
import { TLS, CryptoProviderType } from '@lib/crypto'

const tlsDefault = new TLS() // Uses Ring
const tlsOpenSSL = new TLS(CryptoProviderType.OpenSSL)
const tlsBoring = new TLS(CryptoProviderType.BoringSSL)
const tlsMbed = new TLS(CryptoProviderType.MbedTLS)
```

## Local Development & Building

- Install dependencies:

```bash
npm install
```

- Build host binary:

```bash
npm run build
```

- Local multi-target cross-builds:

```bash
node build.mjs --all
node build.mjs --target aarch64-unknown-linux-gnu --use-napi-cross
```

- Run test suite:

```bash
npm test
```
