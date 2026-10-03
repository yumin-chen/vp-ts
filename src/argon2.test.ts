import { expect, test } from "vite-plus/test";
import {
  argon2Hash,
  argon2HashSync,
  argon2Verify,
  argon2VerifySync,
  argon2ParseOptions,
  argon2HashRawSync,
} from "../dist/index.js";

test("argon2 hashSync and verifySync", () => {
  const password = "my-secret-password";
  const hash = argon2HashSync(password);
  expect(typeof hash).toBe("string");
  expect(hash.startsWith("$argon2id$")).toBe(true);

  const isValid = argon2VerifySync(hash, password);
  expect(isValid).toBe(true);

  const isInvalid = argon2VerifySync(hash, "wrong-password");
  expect(isInvalid).toBe(false);
});

test("argon2 async hash and verify", async () => {
  const password = "async-password";
  const hash = await argon2Hash(password);
  expect(typeof hash).toBe("string");

  const isValid = await argon2Verify(hash, password);
  expect(isValid).toBe(true);

  const isInvalid = await argon2Verify(hash, "wrong-password");
  expect(isInvalid).toBe(false);
});

test("argon2 parseOptions", () => {
  const password = "test-password";
  const hash = argon2HashSync(password);
  const parsed = argon2ParseOptions(hash);
  expect(parsed).toBeDefined();
  expect(typeof parsed.memoryCost).toBe("number");
  expect(typeof parsed.timeCost).toBe("number");
  expect(typeof parsed.parallelism).toBe("number");
});

test("argon2 hashRawSync", () => {
  const password = "raw-password";
  const rawHash = argon2HashRawSync(password);
  expect(Buffer.isBuffer(rawHash)).toBe(true);
  expect(rawHash.length).toBeGreaterThan(0);
});
