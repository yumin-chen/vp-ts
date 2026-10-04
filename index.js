import { createRequire } from "node:module";
const require = createRequire(import.meta.url);

let nativeBinding = null;

try {
  nativeBinding = require("./dist/lib_crypto_native.node");
} catch (e1) {
  try {
    nativeBinding = require("./lib_crypto_native.node");
  } catch (e2) {
    throw new Error(`Failed to load native binding: ${e1.message}; ${e2.message}`);
  }
}

export const {
  argon2,
  argon2Sync,
  Cipher,
  Decipher,
  createCipheriv,
  createDecipheriv,
  Ecdh,
  ECDH,
  createECDH,
  createDiffieHellman,
  createDiffieHellmanGroup,
  Hash,
  createHash,
  hash,
  getHashes,
  Hmac,
  createHmac,
  createMac,
  KeyObject,
  CryptoKeyPair,
  X509Certificate,
  createPublicKey,
  createPrivateKey,
  createSecretKey,
  encapsulate,
  decapsulate,
  diffieHellman,
  generateKeyPair,
  generateKeyPairSync,
  publicEncrypt,
  privateDecrypt,
  hkdf,
  hkdfSync,
  parsePkcs8,
  exportPkcs8,
  pbkdf2,
  pbkdf2Sync,
  randomBytes,
  randomFill,
  randomFillSync,
  randomInt,
  randomUUID,
  randomUUIDv7,
  Sign,
  createSign,
  sign,
  Verify,
  createVerify,
  verify,
  Tls,
  TLS,
} = nativeBinding;

export default nativeBinding;
