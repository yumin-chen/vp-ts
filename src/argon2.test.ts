import assert from "node:assert/strict";
import test from "node:test";
import pkg from "../index.js";

const { argon2, argon2Sync } = pkg;

void test("argon2Sync", () => {
  const message = Buffer.from("password");
  const nonce = Buffer.from("somesalt12345678");
  const res = argon2Sync("argon2id", {
    message,
    nonce,
    parallelism: 1,
    tagLength: 32,
    memory: 4096,
    passes: 3,
  });
  assert.equal(res.length, 32);
});

void test("argon2 async", async () => {
  const message = Buffer.from("password");
  const nonce = Buffer.from("somesalt12345678");
  const res = await new Promise<Buffer>((resolve, reject) => {
    argon2(
      "argon2id",
      { message, nonce, parallelism: 1, tagLength: 32, memory: 4096, passes: 3 },
      (err: Error | null, result?: Buffer) => {
        if (err) reject(err);
        else resolve(result!);
      },
    );
  });
  assert.equal(res.length, 32);
});
