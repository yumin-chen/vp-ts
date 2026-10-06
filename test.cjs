const assert = require("node:assert/strict");
const test = require("node:test");

void test("crypto exports functions", async () => {
  const { createHmac, hash, argon2Sync } = await import("./index.js");
  const hmac = createHmac("sha256", "secret");
  hmac.update("hello");
  assert.equal(typeof hmac.digest("hex"), "string");

  const digest = hash("sha256", "hello", "hex");
  assert.equal(typeof digest, "string");

  const argon = argon2Sync("password", "saltsaltsalt");
  assert.equal(Buffer.isBuffer(argon), true);
});
