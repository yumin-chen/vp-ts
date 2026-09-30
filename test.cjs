const assert = require("node:assert/strict");
const test = require("node:test");

void test("Ksuid and KsuidMs in CJS", async () => {
  const { Ksuid, KsuidMs } = await import("./index.js");

  const ksuid = Ksuid.now();
  assert.equal(ksuid.toBase62().length, 27);
  assert.equal(ksuid.bytes().length, 20);
  assert.equal(ksuid.payload().length, 16);

  const timestamp = 1621627443;
  const payload = Buffer.alloc(16, 5);
  const k1 = Ksuid.fromSeconds(timestamp, payload);
  const k2 = Ksuid.fromSeconds(timestamp + 100, payload);

  assert.equal(k1.timestampSeconds(), timestamp);
  assert.equal(k1.compare(k2), -1);
  assert.equal(k1.equals(k1), true);

  const km = KsuidMs.now();
  assert.equal(km.toBase62().length, 27);
  assert.equal(km.bytes().length, 20);
  assert.equal(km.payload().length, 15);
});
