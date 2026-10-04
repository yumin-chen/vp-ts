import assert from "node:assert/strict";
import test from "node:test";
import { argon2Sync } from "../index.js";

test("Argon2id hashing", () => {
  const pwd = Buffer.from("password123");
  const salt = Buffer.from("some-salt-123456");
  const hash = argon2Sync(pwd, salt, { outputLength: 32 });
  assert.equal(hash.length, 32);
});
