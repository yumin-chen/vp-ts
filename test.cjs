const assert = require("node:assert/strict");
const test = require("node:test");

void test("Ksuid and KsuidMs in CJS with options", async () => {
  const { Ksuid, KsuidMs } = await import("./index.js");

  const ksuid = Ksuid.now({ enc: "base32" });
  assert.equal(ksuid.toString().length, 32);
  assert.equal(ksuid.bytes().length, 20);

  const k48 = new Ksuid(1621627443123, Buffer.alloc(14, 1), { timestampSize: "48bit" });
  assert.equal(k48.payload().length, 14);
  assert.equal(k48.timestampMs(), 1621627443123);

  const km = KsuidMs.now();
  assert.equal(km.toBase62().length, 27);
  assert.equal(km.payload().length, 12);
});
