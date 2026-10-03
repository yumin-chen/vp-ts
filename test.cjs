const assert = require("node:assert/strict");
const test = require("node:test");

void test("crypto hasher works", async () => {
  const { hash, getHashes } = await import("./index.js");
  assert.ok(getHashes().includes("sha256"));
  assert.equal(typeof hash("sha256", "hello", "hex"), "string");
});
