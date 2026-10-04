const assert = require("node:assert/strict");
const test = require("node:test");

const { getHashes, createHash, randomBytes } = require("./index.js");

test("crypto getHashes includes sha256", () => {
  const hashes = getHashes();
  assert.ok(hashes.includes("sha256"));
});

test("crypto createHash sha256", () => {
  const h = createHash("sha256");
  h.update(Buffer.from("hello"));
  assert.equal(h.digest("hex"), "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
});

test("crypto randomBytes", () => {
  const buf = randomBytes(16);
  assert.equal(buf.length, 16);
});
