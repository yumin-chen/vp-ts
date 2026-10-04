import { expect, test } from "vite-plus/test";
import { argon2Sync, argon2VerifySync } from "../index.js";

test("argon2Sync hashes and verifies passwords", () => {
  const password = Buffer.from("my-password");
  const salt = Buffer.alloc(16, 5);

  const hash = argon2Sync(password, salt);
  expect(typeof hash).toBe("string");
  expect(hash).toContain("$argon2");

  const valid = argon2VerifySync(hash, password);
  expect(valid).toBe(true);
});
