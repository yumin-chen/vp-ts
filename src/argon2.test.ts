import { expect, test } from "vite-plus/test";
import {
  argon2Hash,
  argon2HashSync,
  argon2ParseOptions,
  argon2Verify,
  argon2VerifySync,
} from "../index.js";
import type { Algorithm } from "../index.js";

test("argon2 hash_sync and verify_sync", () => {
  const password = "my-secret-password";
  const hashed = argon2HashSync(password);
  expect(typeof hashed).toBe("string");
  expect(hashed.length).toBeGreaterThan(0);

  const isValid = argon2VerifySync(hashed, password);
  expect(isValid).toBe(true);

  const isInvalid = argon2VerifySync(hashed, "wrong-password");
  expect(isInvalid).toBe(false);
});

test("argon2 async hash and verify", async () => {
  const password = "async-password";
  const hashed = await argon2Hash(password, {
    algorithm: 2 as Algorithm,
  });
  expect(typeof hashed).toBe("string");

  const isValid = await argon2Verify(hashed, password);
  expect(isValid).toBe(true);
});

test("argon2 parse_options", () => {
  const hashed = argon2HashSync("password-to-parse", {
    memoryCost: 4096,
    timeCost: 3,
  });
  const options = argon2ParseOptions(hashed);
  expect(options.memoryCost).toBe(4096);
  expect(options.timeCost).toBe(3);
});
