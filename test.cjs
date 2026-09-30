const assert = require("node:assert/strict");
const test = require("node:test");

void test("add", async () => {
  const { add } = await import("./index.js");
  assert.equal(add(2, 3), 5);
});

void test("btls init and version", async () => {
  const { init, version } = await import("./index.js");
  init();
  const v = version();
  assert.equal(typeof v, "string");
  assert.ok(v.length > 0);
});

void test("btls base64 encode and decode", async () => {
  const { base64Encode, base64Decode } = await import("./index.js");
  const input = Buffer.from("Hello BoringSSL!");
  const encoded = base64Encode(input);
  assert.equal(typeof encoded, "string");
  const decoded = base64Decode(encoded);
  assert.deepEqual(decoded, input);
});

void test("btls hash sha256", async () => {
  const { hash } = await import("./index.js");
  const input = Buffer.from("Hello BoringSSL!");
  const h = hash("sha256", input);
  assert.ok(Buffer.isBuffer(h));
  assert.equal(h.length, 32);
});

void test("btls encrypt and decrypt", async () => {
  const { encrypt, decrypt } = await import("./index.js");
  const key = Buffer.from("0123456789abcdef0123456789abcdef"); // 32 bytes
  const iv = Buffer.from("0123456789abcdef"); // 16 bytes
  const data = Buffer.from("Secret payload for btls codec testing!");

  const encrypted = encrypt("aes-256-cbc", key, iv, data);
  assert.ok(Buffer.isBuffer(encrypted));
  assert.notDeepEqual(encrypted, data);

  const decrypted = decrypt("aes-256-cbc", key, iv, encrypted);
  assert.deepEqual(decrypted, data);
});

void test("btls BoringCodec class", async () => {
  const { BoringCodec } = await import("./index.js");
  const key = Buffer.from("0123456789abcdef0123456789abcdef"); // 32 bytes
  const iv = Buffer.from("0123456789abcdef"); // 16 bytes
  const data = Buffer.from("BoringCodec test message");

  const codec = new BoringCodec("aes-256-cbc", key, iv);
  const encoded = codec.encode(data);
  assert.ok(Buffer.isBuffer(encoded));

  const decoded = codec.decode(encoded);
  assert.deepEqual(decoded, data);
});
