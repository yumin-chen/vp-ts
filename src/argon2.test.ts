import { expect, test } from "vite-plus/test";
import { argon2, argon2Sync } from "../index.js";

test("argon2Sync with node:crypto parameters", () => {
  const parameters = {
    message: "password",
    nonce: Buffer.from("1234567890123456"),
    parallelism: 1,
    tagLength: 32,
    memory: 4096,
    passes: 3,
  };
  const derivedKey = argon2Sync("argon2id", parameters);
  expect(Buffer.isBuffer(derivedKey)).toBe(true);
  expect(derivedKey.length).toBe(32);
});

test("argon2 async callback with node:crypto parameters", async () => {
  const parameters = {
    message: "password",
    nonce: Buffer.from("1234567890123456"),
    parallelism: 1,
    tagLength: 64,
    memory: 4096,
    passes: 3,
  };
  const derivedKey = await new Promise((resolve, reject) => {
    argon2("argon2id", parameters, (err, key) => {
      if (err) reject(err);
      else resolve(key);
    });
  });
  expect(Buffer.isBuffer(derivedKey)).toBe(true);
  expect((derivedKey as Buffer).length).toBe(64);
});
