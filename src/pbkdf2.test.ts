import assert from "node:assert/strict";
import test from "node:test";
import pkg from "../index.js";

const { pbkdf2Sync, pbkdf2 } = pkg;

void test("pbkdf2Sync", () => {
  const pwd = Buffer.from("secret");
  const salt = Buffer.from("salt");
  const key = pbkdf2Sync(pwd, salt, 1000, 32, "sha256");
  assert.equal(key.length, 32);
});

void test("pbkdf2 async", async () => {
  const pwd = Buffer.from("secret");
  const salt = Buffer.from("salt");
  const key = await pbkdf2(pwd, salt, 1000, 32, "sha256");
  assert.equal(key.length, 32);
});
