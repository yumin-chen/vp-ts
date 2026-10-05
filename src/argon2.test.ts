import { expect, test } from "vite-plus/test";
import { argon2Sync } from "../artifacts/index.js";

test("argon2Sync computes hash buffer", () => {
  const hash = argon2Sync("argon2id", {
    message: "password",
    nonce: "somesalt123",
    parallelism: 1,
    tagLength: 32,
    memory: 1024,
    passes: 1,
  });
  expect(Buffer.isBuffer(hash)).toBe(true);
  expect(hash.length).toBe(32);
});
