const assert = require("node:assert/strict");
const test = require("node:test");

void test("btls exports work correctly in CJS test", async () => {
  const { BoringSshCodec, encodeBase64, decodeBase64, sha256, encrypt, decrypt, version } =
    await import("./index.js");

  assert.ok(typeof version() === "string");

  const input = Buffer.from("CJS Test Input");
  const encoded = encodeBase64(input);
  const decoded = decodeBase64(encoded);
  assert.equal(decoded.toString("utf-8"), "CJS Test Input");

  const hash = sha256(input);
  assert.equal(hash.length, 32);

  const key = Buffer.alloc(16, 3);
  const iv = Buffer.alloc(16, 4);
  const encrypted = encrypt("aes-128-cbc", key, iv, input);
  const decrypted = decrypt("aes-128-cbc", key, iv, encrypted);
  assert.equal(decrypted.toString("utf-8"), "CJS Test Input");

  const codec = new BoringSshCodec();
  assert.equal(codec.decode(codec.encode(input)).toString("utf-8"), "CJS Test Input");
});
