import { createHash as nodeCreateHash, timingSafeEqual as nodeTimingSafeEqual } from "node:crypto";
import { expect, test } from "vite-plus/test";
import {
  CryptoHasher,
  createHash,
  cryptoHash,
  getHashes,
  getMacs,
  getCiphers,
  getCurves,
  timingSafeEqual,
} from "../dist/index.js";

test("CryptoHasher sha256 matches node:crypto createHash", () => {
  const data = "Hello, world!";
  const hasher = createHash("sha256");
  hasher.update(data);
  const nativeDigest = hasher.digest("hex");

  const nodeHasher = nodeCreateHash("sha256");
  nodeHasher.update(data);
  const nodeDigest = nodeHasher.digest("hex");

  expect(nativeDigest).toBe(nodeDigest);
});

test("CryptoHasher class constructor and sha512", () => {
  const data = "Testing CryptoHasher";
  const hasher = new CryptoHasher("sha512");
  hasher.update(data);
  const nativeDigest = hasher.digest("hex");

  const nodeHasher = nodeCreateHash("sha512");
  nodeHasher.update(data);
  expect(nativeDigest).toBe(nodeHasher.digest("hex"));
});

test("cryptoHash helper function", () => {
  const data = "One shot hash";
  const nativeDigest = cryptoHash("sha256", data, "hex");

  const nodeHasher = nodeCreateHash("sha256");
  nodeHasher.update(data);
  expect(nativeDigest).toBe(nodeHasher.digest("hex"));
});

test("getHashes, getMacs, getCiphers, getCurves", () => {
  expect(getHashes()).toContain("sha256");
  expect(getMacs()).toContain("hmac");
  expect(getCiphers()).toContain("aes-256-gcm");
  expect(getCurves()).toContain("prime256v1");
});

test("timingSafeEqual matches node:crypto", () => {
  const buf1 = Buffer.from("a-secret-value");
  const buf2 = Buffer.from("a-secret-value");
  const buf3 = Buffer.from("a-wrong--value");

  expect(timingSafeEqual(buf1, buf2)).toBe(true);
  expect(timingSafeEqual(buf1, buf3)).toBe(false);
  expect(nodeTimingSafeEqual(buf1, buf2)).toBe(true);
  expect(nodeTimingSafeEqual(buf1, buf3)).toBe(false);

  const diffLen = Buffer.from("different-length");
  expect(() => timingSafeEqual(buf1, diffLen)).toThrow();
});
