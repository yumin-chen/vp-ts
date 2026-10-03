const assert = require("node:assert/strict");
const test = require("node:test");

void test("crypto getHashes and TLS available providers", async () => {
  const { getHashes, TLS, createHash } = await import("./index.js");
  assert.ok(Array.isArray(getHashes()));
  assert.ok(getHashes().includes("sha256"));
  assert.deepEqual(TLS.getAvailableProviders(), ["ring", "openssl", "btls", "mbedtls"]);
  assert.equal(
    createHash("sha256").update("hello").digest("hex"),
    "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
  );
});
