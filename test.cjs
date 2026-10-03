const assert = require("node:assert/strict");
const test = require("node:test");

const {
  createHmac,
  createHash,
  pbkdf2Sync,
  createTls,
  argon2HashSync,
} = require("./dist/index.js");

test("createHmac sha256", () => {
  const hmac = createHmac("sha256", "key");
  hmac.update("message");
  assert.equal(typeof hmac.digest("hex"), "string");
});

test("createHash sha256", () => {
  const hasher = createHash("sha256");
  hasher.update("hello");
  assert.equal(typeof hasher.digest("hex"), "string");
});

test("pbkdf2Sync sha512", () => {
  const key = pbkdf2Sync("password", "salt", 100, 32, "sha512");
  assert.equal(Buffer.isBuffer(key), true);
  assert.equal(key.length, 32);
});

test("createTls default", () => {
  const tls = createTls();
  assert.equal(tls.providerName(), "ring");
});

test("argon2HashSync", () => {
  const hash = argon2HashSync("secret");
  assert.equal(typeof hash, "string");
  assert.equal(hash.startsWith("$argon2id$"), true);
});
