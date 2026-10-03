const assert = require("node:assert/strict");
const test = require("node:test");

void test("exports crypto functions", async () => {
  const { createHash, randomBytes, TLS } = await import("./index.js");
  assert.equal(typeof createHash, "function");
  assert.equal(randomBytes(16).length, 16);
  const tls = new TLS("ring");
  assert.equal(tls.getProviderName(), "ring");
});
