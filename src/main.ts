import nodeCrypto from "node:crypto";
export * from "../build/index.js";
import {
  CryptoHasher,
  ECDH,
  Hmac,
  KeyObject,
  Sign,
  Verify,
  X509Certificate,
  argon2 as nativeArgon2,
  argon2Sync as nativeArgon2Sync,
  getHashes as nativeGetHashes,
  hash as nativeHash,
  pbkdf2 as nativePbkdf2,
  pbkdf2Sync as nativePbkdf2Sync,
  privateDecrypt as nativePrivateDecrypt,
  privateEncrypt as nativePrivateEncrypt,
  publicDecrypt as nativePublicDecrypt,
  publicEncrypt as nativePublicEncrypt,
} from "../build/index.js";

// Re-export native Argon2
export const argon2 = nativeArgon2;
export const argon2Sync = nativeArgon2Sync;

// Re-export PBKDF2
export const pbkdf2 = nativePbkdf2;
export const pbkdf2Sync = nativePbkdf2Sync;

// Re-export RSA
export const publicEncrypt = nativePublicEncrypt;
export const privateDecrypt = nativePrivateDecrypt;
export const publicDecrypt = nativePublicDecrypt;
export const privateEncrypt = nativePrivateEncrypt;

// Factory Functions
export function createHash(algorithm: string, _options?: any) {
  return new CryptoHasher(algorithm);
}

export function createHmac(algorithm: string, key: string | Buffer, _options?: any) {
  return new Hmac(algorithm, key);
}

export function createECDH(curveName: string) {
  return new ECDH(curveName);
}

export function createSign(algorithm: string, _options?: any) {
  return new Sign(algorithm);
}

export function createVerify(algorithm: string, _options?: any) {
  return new Verify(algorithm);
}

export function createMac(algorithm: string, key: any, _options?: any) {
  return new Hmac(algorithm, key);
}

export function createCipheriv(algorithm: string, key: Buffer, iv: Buffer, options?: any) {
  return nodeCrypto.createCipheriv(algorithm, key, iv, options);
}

export function createDecipheriv(algorithm: string, key: Buffer, iv: Buffer, options?: any) {
  return nodeCrypto.createDecipheriv(algorithm, key, iv, options);
}

export function createPrivateKey(key: any) {
  return nodeCrypto.createPrivateKey(key);
}

export function createPublicKey(key: any) {
  return nodeCrypto.createPublicKey(key);
}

export function createSecretKey(key: any, encoding?: string) {
  return nodeCrypto.createSecretKey(key, encoding as any);
}

export function createDiffieHellman(
  sizeOrKey: any,
  keyEncoding?: any,
  generator?: any,
  genEncoding?: any,
) {
  return nodeCrypto.createDiffieHellman(sizeOrKey, keyEncoding, generator, genEncoding);
}

export function createDiffieHellmanGroup(name: string) {
  return nodeCrypto.createDiffieHellmanGroup(name);
}

export const getDiffieHellman = createDiffieHellmanGroup;

// Utilities
export const randomBytes = nodeCrypto.randomBytes;
export const randomFill = nodeCrypto.randomFill;
export const randomFillSync = nodeCrypto.randomFillSync;
export const randomInt = nodeCrypto.randomInt;
export const randomUUID = nodeCrypto.randomUUID;
export const randomUUIDv7 = (nodeCrypto as any).randomUUIDv7 || nodeCrypto.randomUUID;
export const scrypt = nodeCrypto.scrypt;
export const scryptSync = nodeCrypto.scryptSync;
export const hkdf = nodeCrypto.hkdf;
export const hkdfSync = nodeCrypto.hkdfSync;
export const timingSafeEqual = nodeCrypto.timingSafeEqual;
export const checkPrime = nodeCrypto.checkPrime;
export const checkPrimeSync = nodeCrypto.checkPrimeSync;
export const generatePrime = nodeCrypto.generatePrime;
export const generatePrimeSync = nodeCrypto.generatePrimeSync;
export const generateKey = nodeCrypto.generateKey;
export const generateKeySync = nodeCrypto.generateKeySync;
export const generateKeyPair = nodeCrypto.generateKeyPair;
export const generateKeyPairSync = nodeCrypto.generateKeyPairSync;
export const getCiphers = nodeCrypto.getCiphers;
export const getCipherInfo = nodeCrypto.getCipherInfo;
export const getCurves = nodeCrypto.getCurves;
export const getHashes = nativeGetHashes;
export const getMacs = (nodeCrypto as any).getMacs || (() => ["hmac"]);
export const getFips = nodeCrypto.getFips;
export const setFips = nodeCrypto.setFips;
export const hash = nativeHash;
export const encapsulate = (nodeCrypto as any).encapsulate;
export const decapsulate = (nodeCrypto as any).decapsulate;
export const parsePKCS12 = (nodeCrypto as any).parsePKCS12;
export const sign = nodeCrypto.sign;
export const verify = nodeCrypto.verify;

// Classes & Constants
export const Certificate = nodeCrypto.Certificate;
export const Cipheriv = nodeCrypto.Cipheriv;
export const Decipheriv = nodeCrypto.Decipheriv;
export const DiffieHellman = nodeCrypto.DiffieHellman;
export const DiffieHellmanGroup = nodeCrypto.DiffieHellmanGroup;
export const Hash = CryptoHasher;
export const Mac = Hmac;
export const constants = nodeCrypto.constants;
export const webcrypto = nodeCrypto.webcrypto;
export const subtle = nodeCrypto.subtle;
export const getRandomValues = nodeCrypto.getRandomValues;
export const secureHeapUsed = nodeCrypto.secureHeapUsed;

export default {
  argon2,
  argon2Sync,
  checkPrime,
  checkPrimeSync,
  createCipheriv,
  createDecipheriv,
  createDiffieHellman,
  createDiffieHellmanGroup,
  createECDH,
  createHash,
  createHmac,
  createMac,
  createPrivateKey,
  createPublicKey,
  createSecretKey,
  createSign,
  createVerify,
  decapsulate,
  diffieHellman: nodeCrypto.diffieHellman,
  encapsulate,
  generateKey,
  generateKeyPair,
  generateKeyPairSync,
  generateKeySync,
  generatePrime,
  generatePrimeSync,
  getCipherInfo,
  getCiphers,
  getCurves,
  getDiffieHellman,
  getFips,
  getHashes,
  getMacs,
  getRandomValues,
  hash,
  hkdf,
  hkdfSync,
  parsePKCS12,
  pbkdf2,
  pbkdf2Sync,
  privateDecrypt,
  privateEncrypt,
  publicDecrypt,
  publicEncrypt,
  randomBytes,
  randomFill,
  randomFillSync,
  randomInt,
  randomUUID,
  randomUUIDv7,
  scrypt,
  scryptSync,
  secureHeapUsed,
  setEngine: nodeCrypto.setEngine,
  setFips,
  sign,
  subtle,
  timingSafeEqual,
  verify,
  webcrypto,

  // Classes
  Certificate,
  Cipheriv,
  Decipheriv,
  DiffieHellman,
  DiffieHellmanGroup,
  ECDH,
  Hash,
  Hmac,
  KeyObject,
  Sign,
  Verify,
  X509Certificate,
  constants,
};
