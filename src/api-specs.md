# `@lib/crypto` API Compatibility Specification

`@lib/crypto` provides a high-performance native Node.js `node:crypto` compatible implementation powered by NAPI-RS and open-source Rust cryptography primitives (`ring`, `rustls`, `argon2`, `aes-gcm`, `rsa`, `x25519-dalek`, `ed25519-dalek`, `x509-parser`).

## Comprehensive `node:crypto` API Mapping & Compatibility Matrix

| Node.js `node:crypto` Class / Method                          | `@lib/crypto` Export              | Native Backend  | Status    |
| :------------------------------------------------------------ | :-------------------------------- | :-------------- | :-------- |
| **`crypto.argon2(algorithm, parameters, callback)`**          | `argon2`                          | `argon2` crate  | Supported |
| **`crypto.argon2Sync(algorithm, parameters)`**                | `argon2Sync`                      | `argon2` crate  | Supported |
| **`crypto.checkPrime(candidate, [options], cb)`**             | `checkPrime`                      | Native          | Supported |
| **`crypto.checkPrimeSync(candidate, [options])`**             | `checkPrimeSync`                  | Native          | Supported |
| **`crypto.constants`**                                        | `constants`                       | Native          | Supported |
| **`crypto.createCipheriv(algorithm, key, iv)`**               | `createCipheriv`                  | `aes-gcm`       | Supported |
| **`crypto.createDecipheriv(algorithm, key, iv)`**             | `createDecipheriv`                | `aes-gcm`       | Supported |
| **`crypto.createDiffieHellman(prime, ...)`**                  | `createDiffieHellman`             | Native          | Supported |
| **`crypto.createECDH(curveName)`**                            | `createECDH` / `createEcdh`       | `p256` / `p384` | Supported |
| **`crypto.createHash(algorithm)`**                            | `createHash`                      | `sha2` / `sha1` | Supported |
| **`crypto.createHmac(algorithm, key)`**                       | `createHmac`                      | `ring`          | Supported |
| **`crypto.createMac(algorithm, key, [options])`**             | `createMac`                       | `ring`          | Supported |
| **`crypto.createPrivateKey(key)`**                            | `createPrivateKey`                | Native          | Supported |
| **`crypto.createPublicKey(key)`**                             | `createPublicKey`                 | Native          | Supported |
| **`crypto.createSecretKey(key)`**                             | `createSecretKey`                 | Native          | Supported |
| **`crypto.createSign(algorithm)`**                            | `createSign`                      | `rsa`           | Supported |
| **`crypto.createVerify(algorithm)`**                          | `createVerify`                    | `rsa`           | Supported |
| **`crypto.decapsulate(key, ciphertext)`**                     | `decapsulate`                     | `x25519-dalek`  | Supported |
| **`crypto.encapsulate(key)`**                                 | `encapsulate`                     | `x25519-dalek`  | Supported |
| **`crypto.generateKeyPair(type, options, cb)`**               | `generateKeyPair`                 | `rsa`           | Supported |
| **`crypto.generateKeyPairSync(type, options)`**               | `generateKeyPairSync`             | `rsa`           | Supported |
| **`crypto.getCipherInfo(name)`**                              | `getCipherInfo`                   | Native          | Supported |
| **`crypto.getCiphers()`**                                     | `getCiphers`                      | Native          | Supported |
| **`crypto.getCurves()`**                                      | `getCurves`                       | Native          | Supported |
| **`crypto.getHashes()`**                                      | `getHashes`                       | Native          | Supported |
| **`crypto.getMacs()`**                                        | `getMacs`                         | Native          | Supported |
| **`crypto.getRandomValues(typedArray)`**                      | `getRandomValues`                 | `ring::rand`    | Supported |
| **`crypto.hash(algorithm, data)`**                            | `hash`                            | `sha2`          | Supported |
| **`crypto.hkdf(digest, ikm, salt, info, keylen, cb)`**        | `hkdf`                            | `hkdf` crate    | Supported |
| **`crypto.hkdfSync(digest, ikm, salt, info, keylen)`**        | `hkdfSync`                        | `hkdf` crate    | Supported |
| **`crypto.pbkdf2(password, salt, iter, keylen, digest, cb)`** | `pbkdf2`                          | `pbkdf2` crate  | Supported |
| **`crypto.pbkdf2Sync(password, salt, iter, keylen, digest)`** | `pbkdf2Sync`                      | `pbkdf2` crate  | Supported |
| **`crypto.privateDecrypt(privateKey, buffer)`**               | `privateDecrypt`                  | `rsa`           | Supported |
| **`crypto.privateEncrypt(privateKey, buffer)`**               | `privateEncrypt`                  | `rsa`           | Supported |
| **`crypto.publicDecrypt(key, buffer)`**                       | `publicDecrypt`                   | `rsa`           | Supported |
| **`crypto.publicEncrypt(key, buffer)`**                       | `publicEncrypt`                   | `rsa`           | Supported |
| **`crypto.randomBytes(size, [cb])`**                          | `randomBytes`                     | `ring::rand`    | Supported |
| **`crypto.randomFill(buffer, offset, size, cb)`**             | `randomFill`                      | `ring::rand`    | Supported |
| **`crypto.randomFillSync(buffer, offset, size)`**             | `randomFillSync`                  | `ring::rand`    | Supported |
| **`crypto.randomInt([min,] max, [cb])`**                      | `randomInt`                       | `ring::rand`    | Supported |
| **`crypto.randomUUID([options])`**                            | `randomUuid` / `randomUUID`       | `uuid` v4       | Supported |
| **`crypto.randomUUIDv7([options])`**                          | `randomUuidv7`                    | `uuid` v7       | Supported |
| **`crypto.scrypt(password, salt, keylen, [opts], cb)`**       | `scrypt`                          | `pbkdf2` crate  | Supported |
| **`crypto.scryptSync(password, salt, keylen, [opts])`**       | `scryptSync`                      | `pbkdf2` crate  | Supported |
| **`crypto.sign(algorithm, data, key)`**                       | `sign`                            | `rsa`           | Supported |
| **`crypto.timingSafeEqual(a, b)`**                            | `timingSafeEqual`                 | Native          | Supported |
| **`crypto.verify(algorithm, data, key, signature)`**          | `verify`                          | `rsa`           | Supported |
| **`Class: Certificate`**                                      | `Certificate`                     | Native          | Supported |
| **`Class: Cipheriv`**                                         | `Cipheriv` / `createCipheriv`     | `aes-gcm`       | Supported |
| **`Class: Decipheriv`**                                       | `Decipheriv` / `createDecipheriv` | `aes-gcm`       | Supported |
| **`Class: ECDH`**                                             | `Ecdh` / `ECDH`                   | `p256` / `p384` | Supported |
| **`Class: Hash`**                                             | `Hash`                            | `sha2`          | Supported |
| **`Class: Hmac`**                                             | `Hmac`                            | `ring`          | Supported |
| **`Class: KeyObject`**                                        | `KeyObject`                       | Native          | Supported |
| **`Class: Sign`**                                             | `Sign`                            | `rsa`           | Supported |
| **`Class: Verify`**                                           | `Verify`                          | `rsa`           | Supported |
| **`Class: X509Certificate`**                                  | `X509Certificate`                 | `x509-parser`   | Supported |
| **`Class: TLS`**                                              | `Tls` / `TLS`                     | `rustls`        | Supported |
