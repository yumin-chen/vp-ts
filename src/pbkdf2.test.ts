import { expect, test } from "vite-plus/test";
import { pbkdf2, pbkdf2Sync } from "../index.js";

test("pbkdf2Sync derives expected key", () => {
  const key = pbkdf2Sync("secret", "salt", 1000, 32, "sha256");
  expect(Buffer.isBuffer(key)).toBe(true);
  expect(key.length).toBe(32);
});

test("pbkdf2 derives key", () => {
  const key = pbkdf2("secret", "salt", 1000, 64, "sha512");
  expect(Buffer.isBuffer(key)).toBe(true);
  expect(key.length).toBe(64);
});
