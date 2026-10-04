import { expect, test } from "vite-plus/test";
import {
  argon2Hash,
  argon2HashSync,
  argon2Verify,
  argon2VerifySync,
  argon2ParseOptions,
  argon2HashRawSync,
  argon2Sync,
} from "../dist/index.js";

const fastOptions = { memoryCost: 512, timeCost: 1 };

test("argon2 hashSync and verifySync", () => {
  const password = "my-secret-password";
  const hash = argon2HashSync(password, fastOptions);
  expect(typeof hash).toBe("string");
  expect(hash.startsWith("$argon2id$")).toBe(true);

  const isValid = argon2VerifySync(hash, password, fastOptions);
  expect(isValid).toBe(true);

  const isInvalid = argon2VerifySync(hash, "wrong-password", fastOptions);
  expect(isInvalid).toBe(false);
});

test("argon2 async hash and verify", async () => {
  const password = "async-password";
  const hash = await argon2Hash(password, fastOptions);
  expect(typeof hash).toBe("string");

  const isValid = await argon2Verify(hash, password, fastOptions);
  expect(isValid).toBe(true);

  const isInvalid = await argon2Verify(hash, "wrong-password", fastOptions);
  expect(isInvalid).toBe(false);
});

test("argon2 parseOptions", () => {
  const password = "test-password";
  const hash = argon2HashSync(password, fastOptions);
  const parsed = argon2ParseOptions(hash);
  expect(parsed).toBeDefined();
  expect(typeof parsed.memoryCost).toBe("number");
  expect(typeof parsed.timeCost).toBe("number");
  expect(typeof parsed.parallelism).toBe("number");
});

test("argon2 hashRawSync", () => {
  const password = "raw-password";
  const rawHash = argon2HashRawSync(password, fastOptions);
  expect(Buffer.isBuffer(rawHash)).toBe(true);
  expect(rawHash.length).toBeGreaterThan(0);
});

test("argon2Sync node:crypto API", () => {
  const params = {
    message: "node-password",
    nonce: "salt-nonce-12345",
    parallelism: 1,
    tagLength: 32,
    memory: 512,
    passes: 1,
  };
  const key = argon2Sync("argon2id", params);
  expect(Buffer.isBuffer(key)).toBe(true);
  expect(key.length).toBe(32);
});
