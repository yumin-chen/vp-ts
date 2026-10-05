const assert = require("node:assert/strict");
const test = require("node:test");

const { createHmac, randomBytes, createHash, getHashes } = require("./index.js");

test("@lib/crypto basic exports", () => {
  const bytes = randomBytes(16);
  assert.equal(bytes.length, 16);

  const hmac = createHmac("sha256", Buffer.from("key"));
  hmac.update(Buffer.from("data"));
  assert.equal(typeof hmac.digest("hex"), "string");

  const hash = createHash("sha256");
  hash.update(Buffer.from("data"));
  assert.equal(typeof hash.digest("hex"), "string");

  assert.ok(getHashes().includes("sha256"));
});
