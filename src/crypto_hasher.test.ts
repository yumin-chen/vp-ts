import { expect, test } from "vite-plus/test";
import {
  Hash,
  createHash,
  hash,
  randomBytes,
  randomFillSync,
  randomUuid,
  timingSafeEqual,
  getHashes,
  getCiphers,
  getCurves,
  getMacs,
  hkdfSync,
} from "../artifacts/index.js";

test("Hash class computes SHA256 digest", () => {
  const h = new Hash("sha256");
  h.update("Node.js");
  const digest = h.digest("hex");
  expect(typeof digest).toBe("string");
  expect(digest.length).toBe(64);
});

test("createHash and hash utility function work correctly", () => {
  const d1 = hash("sha256", "test data", "hex");
  const h = createHash("sha256");
  h.update("test data");
  const d2 = h.digest("hex");
  expect(d1).toBe(d2);
});

test("randomBytes generates random buffer of requested size", () => {
  const buf = randomBytes(16);
  expect(Buffer.isBuffer(buf)).toBe(true);
  expect(buf.length).toBe(16);
});

test("randomFillSync fills existing buffer", () => {
  const buf = Buffer.alloc(10);
  randomFillSync(buf);
  expect(buf.some((byte) => byte !== 0)).toBe(true);
});

test("randomUuid returns valid v4 UUID string", () => {
  const uuid = randomUuid();
  expect(typeof uuid).toBe("string");
  expect(uuid).toMatch(/^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i);
});

test("timingSafeEqual correctly compares buffers", () => {
  const a = Buffer.from("secret123");
  const b = Buffer.from("secret123");
  const c = Buffer.from("diffent12");

  expect(timingSafeEqual(a, b)).toBe(true);
  expect(timingSafeEqual(a, c)).toBe(false);
});

test("getHashes, getCiphers, getCurves, getMacs return non-empty lists", () => {
  expect(getHashes()).toContain("sha256");
  expect(getCiphers()).toContain("aes-256-gcm");
  expect(getCurves()).toContain("p256");
  expect(getMacs()).toContain("hmac");
});

test("hkdfSync computes derived key", () => {
  const key = hkdfSync("sha256", "ikm", "salt", "info", 32);
  expect(Buffer.isBuffer(key)).toBe(true);
  expect(key.length).toBe(32);
});
