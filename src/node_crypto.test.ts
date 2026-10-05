import { describe, expect, it } from "vite-plus/test";
import crypto, {
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
  checkPrime,
  checkPrimeSync,
  constants,
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
  hkdf,
  hkdfSync,
  randomBytes,
  randomFill,
  randomFillSync,
  randomInt,
  randomUUID,
  scrypt,
  scryptSync,
  setFips,
  subtle,
  timingSafeEqual,
  webcrypto,
} from "./main.js";

describe("node:crypto full module API contract", () => {
  it("exports all required factory methods", () => {
    expect(typeof createHash).toBe("function");
    expect(typeof createHmac).toBe("function");
    expect(typeof createECDH).toBe("function");
    expect(typeof createSign).toBe("function");
    expect(typeof createVerify).toBe("function");
    expect(typeof createMac).toBe("function");
    expect(typeof createCipheriv).toBe("function");
    expect(typeof createDecipheriv).toBe("function");
    expect(typeof createPrivateKey).toBe("function");
    expect(typeof createPublicKey).toBe("function");
    expect(typeof createSecretKey).toBe("function");
    expect(typeof createDiffieHellman).toBe("function");
    expect(typeof createDiffieHellmanGroup).toBe("function");
    expect(typeof getDiffieHellman).toBe("function");
  });

  it("exports all utility functions", () => {
    expect(typeof randomBytes).toBe("function");
    expect(typeof randomFill).toBe("function");
    expect(typeof randomFillSync).toBe("function");
    expect(typeof randomInt).toBe("function");
    expect(typeof randomUUID).toBe("function");
    expect(typeof scrypt).toBe("function");
    expect(typeof scryptSync).toBe("function");
    expect(typeof hkdf).toBe("function");
    expect(typeof hkdfSync).toBe("function");
    expect(typeof timingSafeEqual).toBe("function");
    expect(typeof checkPrime).toBe("function");
    expect(typeof checkPrimeSync).toBe("function");
    expect(typeof generatePrime).toBe("function");
    expect(typeof generatePrimeSync).toBe("function");
    expect(typeof generateKey).toBe("function");
    expect(typeof generateKeySync).toBe("function");
    expect(typeof generateKeyPair).toBe("function");
    expect(typeof generateKeyPairSync).toBe("function");
    expect(typeof getCiphers).toBe("function");
    expect(typeof getCipherInfo).toBe("function");
    expect(typeof getCurves).toBe("function");
    expect(typeof getHashes).toBe("function");
    expect(typeof getMacs).toBe("function");
    expect(typeof getFips).toBe("function");
    expect(typeof setFips).toBe("function");
    expect(typeof getRandomValues).toBe("function");
  });

  it("exports all classes and objects", () => {
    expect(Certificate).toBeDefined();
    expect(Cipheriv).toBeDefined();
    expect(Decipheriv).toBeDefined();
    expect(DiffieHellman).toBeDefined();
    expect(DiffieHellmanGroup).toBeDefined();
    expect(ECDH).toBeDefined();
    expect(Hash).toBeDefined();
    expect(Hmac).toBeDefined();
    expect(KeyObject).toBeDefined();
    expect(Sign).toBeDefined();
    expect(Verify).toBeDefined();
    expect(X509Certificate).toBeDefined();
    expect(constants).toBeDefined();
    expect(webcrypto).toBeDefined();
    expect(subtle).toBeDefined();
  });

  it("factory functions create instances", () => {
    const hashInst = createHash("sha256");
    hashInst.update("hello");
    expect(typeof hashInst.digest("hex")).toBe("string");

    const hmacInst = createHmac("sha256", "secret");
    hmacInst.update("hello");
    expect(typeof hmacInst.digest("hex")).toBe("string");

    const ecdhInst = createECDH("p-256");
    expect(Buffer.isBuffer(ecdhInst.getPublicKey())).toBe(true);

    const signInst = createSign("ed25519");
    signInst.update("data");
    expect(signInst).toBeDefined();

    const verifyInst = createVerify("ed25519");
    verifyInst.update("data");
    expect(verifyInst).toBeDefined();
  });

  it("default export matches named exports", () => {
    expect(typeof crypto.createHash).toBe("function");
    expect(typeof crypto.createHmac).toBe("function");
    expect(typeof crypto.argon2).toBe("function");
    expect(typeof crypto.argon2Sync).toBe("function");
  });
});
