import { expect, test } from "vite-plus/test";
import { hkdf, hkdfSync } from "../index.js";

test("hkdfSync derives key of requested length", () => {
  const derived = hkdfSync("sha256", "secret", "salt", "info", 32);
  expect(Buffer.isBuffer(derived)).toBe(true);
  expect(derived.length).toBe(32);
});

test("hkdf function", () => {
  const derived = hkdf("sha256", "secret", "salt", "info", 64);
  expect(Buffer.isBuffer(derived)).toBe(true);
  expect(derived.length).toBe(64);
});
