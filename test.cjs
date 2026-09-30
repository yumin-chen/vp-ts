const assert = require("node:assert/strict");
const test = require("node:test");
const { encode, decode, Extension } = require("./index.js");

test("nil format", () => {
  const buf = encode(null);
  assert.equal(buf[0], 0xc0);
  assert.equal(decode(buf), null);

  const bufUndef = encode(undefined);
  assert.equal(bufUndef[0], 0xc0);
  assert.equal(decode(bufUndef), null);
});

test("bool format family", () => {
  const bufTrue = encode(true);
  assert.equal(bufTrue[0], 0xc3);
  assert.equal(decode(bufTrue), true);

  const bufFalse = encode(false);
  assert.equal(bufFalse[0], 0xc2);
  assert.equal(decode(bufFalse), false);
});

test("int format family", () => {
  // positive fixint
  assert.equal(decode(encode(0)), 0);
  assert.equal(decode(encode(127)), 127);
  // negative fixint
  assert.equal(decode(encode(-1)), -1);
  assert.equal(decode(encode(-31)), -31);

  // unsigned & signed integers
  assert.equal(decode(encode(250)), 250);
  assert.equal(decode(encode(65000)), 65000);
  assert.equal(decode(encode(2000000000)), 2000000000);
  assert.equal(decode(encode(-1000)), -1000);
  assert.equal(decode(encode(-2000000000)), -2000000000);

  // bigint
  assert.equal(decode(encode(9007199254740992n)), 9007199254740992n);
  assert.equal(decode(encode(-9007199254740992n)), -9007199254740992n);
});

test("float format family", () => {
  const val = 3.141592653589793;
  assert.equal(decode(encode(val)), val);

  assert.ok(Number.isNaN(decode(encode(NaN))));
  assert.equal(decode(encode(Infinity)), Infinity);
  assert.equal(decode(encode(-Infinity)), -Infinity);
});

test("str format family", () => {
  const str = "MessagePack 📦 🚀 日本語";
  const buf = encode(str);
  assert.equal(decode(buf), str);

  assert.equal(decode(encode("")), "");
});

test("bin format family", () => {
  const data = new Uint8Array([0, 1, 2, 254, 255]);
  const encoded = encode(data);
  const decoded = decode(encoded);
  assert.ok(decoded instanceof Uint8Array);
  assert.deepEqual(Array.from(decoded), Array.from(data));
});

test("array format family", () => {
  const arr = [1, "hello", true, null, [2, 3]];
  const decoded = decode(encode(arr));
  assert.deepEqual(decoded, arr);
});

test("map format family", () => {
  // Plain Object
  const obj = { name: "Alice", age: 30, active: true, tags: ["admin"] };
  assert.deepEqual(decode(encode(obj)), obj);

  // Map instance with non-string keys
  const map = new Map();
  map.set(1, "one");
  map.set(2, "two");
  const decodedMap = decode(encode(map));
  assert.ok(decodedMap instanceof Map);
  assert.equal(decodedMap.get(1), "one");
  assert.equal(decodedMap.get(2), "two");
});

test("ext format family & Extension class", () => {
  const ext = new Extension(42, new Uint8Array([10, 20, 30]));
  const encoded = encode(ext);
  const decoded = decode(encoded);
  assert.equal(decoded.type, 42);
  assert.deepEqual(Array.from(decoded.data), [10, 20, 30]);
});

test("timestamp extension type", () => {
  const now = new Date(1700000000000);
  const encoded = encode(now);
  const decoded = decode(encoded);
  assert.ok(decoded instanceof Date);
  assert.equal(decoded.getTime(), now.getTime());
});
