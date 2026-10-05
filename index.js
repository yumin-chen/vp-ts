import { createRequire } from "node:module";
const require = createRequire(import.meta.url);

const { readFileSync } = require("node:fs");
let nativeBinding = null;
let __napiLoadedBindingTarget = "native";
const loadErrors = [];

const _isMusl = () => {
  let musl = false;
  if (process.platform === "linux") {
    musl = _isMuslFromFilesystem();
    if (musl === null) {
      musl = _isMuslFromReport();
    }
    if (musl === null) {
      musl = _isMuslFromChildProcess();
    }
  }
  return musl;
};

const _isFileMusl = (f) => f.includes("libc.musl-") || f.includes("ld-musl-");

const _isMuslFromFilesystem = () => {
  try {
    return readFileSync("/usr/bin/ldd", "utf-8").includes("musl");
  } catch {
    return null;
  }
};

const _isMuslFromReport = () => {
  let report = null;
  if (process.report && typeof process.report.getReport === "function") {
    process.report.excludeNetwork = true;
    report = process.report.getReport();
  }
  if (!report) {
    return null;
  }
  if (report.header && report.header.glibcVersionRuntime) {
    return false;
  }
  if (Array.isArray(report.sharedObjects)) {
    if (report.sharedObjects.some(_isFileMusl)) {
      return true;
    }
  }
  return false;
};

const _isMuslFromChildProcess = () => {
  try {
    return require("node:child_process")
      .execSync("ldd --version", { encoding: "utf8" })
      .includes("musl");
  } catch {
    return false;
  }
};

function requireNative() {
  if (process.env.NAPI_RS_NATIVE_LIBRARY_PATH) {
    try {
      const overrideBinding = require(process.env.NAPI_RS_NATIVE_LIBRARY_PATH);
      __napiLoadedBindingTarget =
        overrideBinding && typeof overrideBinding.__napiBindingTarget === "string"
          ? overrideBinding.__napiBindingTarget
          : "native";
      return overrideBinding;
    } catch (err) {
      loadErrors.push(err);
    }
  }

  try {
    return require("./dist/lib_crypto_native.node");
  } catch (err) {
    loadErrors.push(err);
  }

  try {
    return require("./lib_crypto_native.node");
  } catch (err) {
    loadErrors.push(err);
  }
}

nativeBinding = requireNative();

if (!nativeBinding) {
  if (loadErrors.length > 0) {
    const error = new Error("Failed to load native binding for @lib/crypto.");
    error.cause = loadErrors;
    throw error;
  }
}

const wrapCallback = (fn) => {
  return (...args) => {
    const lastArg = args[args.length - 1];
    if (typeof lastArg === "function") {
      const callback = args.pop();
      try {
        const res = fn(...args);
        process.nextTick(() => callback(null, res));
      } catch (err) {
        process.nextTick(() => callback(err));
      }
      return;
    }
    return fn(...args);
  };
};

export const argon2Sync = nativeBinding.argon2Sync;
export const argon2 = (algorithm, parameters, callback) => {
  if (typeof callback === "function") {
    try {
      const res = nativeBinding.argon2(algorithm, parameters);
      process.nextTick(() => callback(null, res));
    } catch (err) {
      process.nextTick(() => callback(err));
    }
    return;
  }
  return nativeBinding.argon2(algorithm, parameters);
};

export const checkPrimeSync = nativeBinding.checkPrimeSync;
export const checkPrime = wrapCallback(nativeBinding.checkPrime);

export const generatePrimeSync = nativeBinding.generatePrimeSync;
export const generatePrime = wrapCallback(nativeBinding.generatePrime);

export const Cipher = nativeBinding.Cipher;
export const Cipheriv = nativeBinding.Cipher;
export const createCipheriv = nativeBinding.createCipheriv;

export const Decipher = nativeBinding.Decipher;
export const Decipheriv = nativeBinding.Decipher;
export const createDecipheriv = nativeBinding.createDecipheriv;

export const Ecdh = nativeBinding.Ecdh;
export const ECDH = nativeBinding.Ecdh;
export const DiffieHellman = nativeBinding.Ecdh;
export const DiffieHellmanGroup = nativeBinding.Ecdh;
export const createECDH = nativeBinding.createECDH;
export const createDiffieHellman = nativeBinding.createDiffieHellman;
export const createDiffieHellmanGroup = nativeBinding.createDiffieHellmanGroup;
export const getDiffieHellman = nativeBinding.createDiffieHellmanGroup;

export const Hash = nativeBinding.Hash;
export const createHash = nativeBinding.createHash;
export const hash = nativeBinding.hash;
export const getHashes = nativeBinding.getHashes;

export const Hmac = nativeBinding.Hmac;
export const createHmac = nativeBinding.createHmac;
export const createMac = nativeBinding.createMac;

export const KeyObject = nativeBinding.KeyObject;
export const CryptoKeyPair = nativeBinding.CryptoKeyPair;
export const X509Certificate = nativeBinding.X509Certificate;
export const Certificate = nativeBinding.Certificate;

export const createPublicKey = nativeBinding.createPublicKey;
export const createPrivateKey = nativeBinding.createPrivateKey;
export const createSecretKey = nativeBinding.createSecretKey;

export const encapsulate = wrapCallback(nativeBinding.encapsulate);
export const decapsulate = wrapCallback(nativeBinding.decapsulate);
export const diffieHellman = wrapCallback(nativeBinding.diffieHellman);

export const generateKeyPairSync = nativeBinding.generateKeyPairSync;
export const generateKeyPair = wrapCallback(nativeBinding.generateKeyPair);

export const generateKeySync = nativeBinding.generateKeySync;
export const generateKey = wrapCallback(nativeBinding.generateKey);

export const getCiphers = nativeBinding.getCiphers;
export const getCipherInfo = nativeBinding.getCipherInfo;
export const getCurves = nativeBinding.getCurves;
export const getMacs = nativeBinding.getMacs;
export const getFips = nativeBinding.getFips;
export const setFips = nativeBinding.setFips;
export const setEngine = nativeBinding.setEngine;
export const secureHeapUsed = nativeBinding.secureHeapUsed;

export const publicEncrypt = nativeBinding.publicEncrypt;
export const publicDecrypt = nativeBinding.publicDecrypt;
export const privateEncrypt = nativeBinding.privateEncrypt;
export const privateDecrypt = nativeBinding.privateDecrypt;

export const hkdfSync = nativeBinding.hkdfSync;
export const hkdf = wrapCallback(nativeBinding.hkdf);

export const parsePkcs8 = nativeBinding.parsePkcs8;
export const exportPkcs8 = nativeBinding.exportPkcs8;
export const parsePKCS12 = nativeBinding.parsePKCS12;

export const pbkdf2Sync = nativeBinding.pbkdf2Sync;
export const pbkdf2 = wrapCallback(nativeBinding.pbkdf2);

export const scryptSync = nativeBinding.scryptSync;
export const scrypt = wrapCallback(nativeBinding.scrypt);

export const randomBytes = wrapCallback(nativeBinding.randomBytes);
export const prng = randomBytes;
export const pseudoRandomBytes = randomBytes;
export const rng = randomBytes;

export const randomFillSync = nativeBinding.randomFillSync;
export const randomFill = wrapCallback(nativeBinding.randomFill);

export const randomInt = wrapCallback(nativeBinding.randomInt);
export const randomUUID = nativeBinding.randomUUID;
export const randomUUIDv7 = nativeBinding.randomUUIDv7;

export const Sign = nativeBinding.Sign;
export const createSign = nativeBinding.createSign;
export const sign = wrapCallback(nativeBinding.sign);

export const Verify = nativeBinding.Verify;
export const createVerify = nativeBinding.createVerify;
export const verify = wrapCallback(nativeBinding.verify);

export const Tls = nativeBinding.Tls;
export const TLS = nativeBinding.Tls;

export const timingSafeEqual = nativeBinding.timingSafeEqual;

export const fips = 0;

export const constants = {
  RSA_PKCS1_PADDING: 1,
  RSA_SSLV23_PADDING: 2,
  RSA_NO_PADDING: 3,
  RSA_PKCS1_OAEP_PADDING: 4,
  RSA_X931_PADDING: 5,
  RSA_PKCS1_PSS_PADDING: 6,
};

export const getRandomValues = (array) => {
  if (!array || !array.byteLength) return array;
  const bytes = randomBytes(array.byteLength);
  const view = new Uint8Array(array.buffer, array.byteOffset, array.byteLength);
  view.set(bytes);
  return array;
};

export const webcrypto = {
  getRandomValues,
  subtle: {},
};

export const subtle = webcrypto.subtle;

export const __napiBindingTarget = __napiLoadedBindingTarget;

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
  diffieHellman,
  encapsulate,
  exportPkcs8,
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
  parsePkcs8,
  pbkdf2,
  pbkdf2Sync,
  privateDecrypt,
  privateEncrypt,
  prng,
  pseudoRandomBytes,
  publicDecrypt,
  publicEncrypt,
  randomBytes,
  randomFill,
  randomFillSync,
  randomInt,
  randomUUID,
  randomUUIDv7,
  rng,
  scrypt,
  scryptSync,
  secureHeapUsed,
  setEngine,
  setFips,
  sign,
  subtle,
  timingSafeEqual,
  verify,
  webcrypto,
  Certificate,
  Cipher,
  Cipheriv,
  CryptoKeyPair,
  Decipher,
  Decipheriv,
  DiffieHellman,
  DiffieHellmanGroup,
  ECDH,
  Ecdh,
  Hash,
  Hmac,
  KeyObject,
  Sign,
  TLS,
  Tls,
  Verify,
  X509Certificate,
  constants,
  fips,
};
