import { expect, test } from "vite-plus/test";
import { scrypt, scryptSync } from "../index.js";

test("scryptSync derives expected key", () => {
  const key = scryptSync("password", "salt", 32);
  expect(Buffer.isBuffer(key)).toBe(true);
  expect(key.length).toBe(32);
});

test("scrypt function derives key", () => {
  const key = scrypt("password", "salt", 64) as Buffer;
  expect(Buffer.isBuffer(key)).toBe(true);
  expect(key.length).toBe(64);
});
