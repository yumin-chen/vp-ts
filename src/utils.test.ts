import { expect, test } from "vite-plus/test";
import {
  timingSafeEqual,
  getCiphers,
  getCipherInfo,
  getCurves,
  getMacs,
  getFips,
  setFips,
  generateKeySync,
  privateEncrypt,
  publicDecrypt,
  parsePKCS12,
  Cipheriv,
  Decipheriv,
  Certificate,
  generateKeyPairSync,
} from "../index.js";

test("timingSafeEqual compares equal and non-equal buffers", () => {
  const buf1 = Buffer.from("hello");
  const buf2 = Buffer.from("hello");
  const buf3 = Buffer.from("world");

  expect(timingSafeEqual(buf1, buf2)).toBe(true);
  expect(timingSafeEqual(buf1, buf3)).toBe(false);
});

test("getCiphers, getCipherInfo, getCurves, getMacs, getFips, setFips", () => {
  expect(getCiphers().length).toBeGreaterThan(0);
  expect(getCipherInfo("aes-256-gcm")).toBeDefined();
  expect(getCurves().length).toBeGreaterThan(0);
  expect(getMacs().length).toBeGreaterThan(0);
  expect(getFips()).toBe(0);
  setFips(true);
});

test("generateKeySync generates secret key", () => {
  const keyObj = generateKeySync("hmac", { length: 256 });
  expect(keyObj.type).toBe("secret");
});

test("privateEncrypt and publicDecrypt", () => {
  const { publicKey, privateKey } = generateKeyPairSync("rsa", { modulusLength: 1024 });
  const msg = Buffer.from("test");
  const encrypted = privateEncrypt(privateKey, msg);
  expect(Buffer.isBuffer(encrypted)).toBe(true);
  const decrypted = publicDecrypt(publicKey, encrypted);
  expect(Buffer.isBuffer(decrypted)).toBe(true);
});

test("parsePKCS12, Cipheriv, Decipheriv, Certificate", () => {
  const parsed = parsePKCS12(Buffer.from("p12"));
  expect(parsed).toBeDefined();

  const key = Buffer.alloc(32, 1);
  const iv = Buffer.alloc(12, 2);
  const c = new Cipheriv("aes-256-gcm", key, iv);
  expect(c).toBeDefined();

  const d = new Decipheriv("aes-256-gcm", key, iv);
  expect(d).toBeDefined();

  expect(Certificate.verifySpkac(Buffer.from("spkac"))).toBe(true);
});
