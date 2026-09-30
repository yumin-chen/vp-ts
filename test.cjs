const assert = require("node:assert/strict");
const test = require("node:test");

const Sqids = require("./index.js");
const { defaultOptions, Sqids: SqidsNamed } = require("./index.js");

test("exports Sqids and defaultOptions", () => {
  assert.equal(typeof Sqids, "function");
  assert.equal(Sqids, SqidsNamed);
  assert.equal(typeof defaultOptions, "object");
  assert.equal(defaultOptions.minLength, 0);
  assert.equal(
    defaultOptions.alphabet,
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
  );
  assert.ok(defaultOptions.blocklist.has("0rgasm"));
});

test("default encode and decode", () => {
  const sqids = new Sqids();
  const id = sqids.encode([1, 2, 3]);
  assert.equal(id, "86Rf07");
  const numbers = sqids.decode(id);
  assert.deepEqual(numbers, [1, 2, 3]);
});

test("minLength option", () => {
  const sqids = new Sqids({ minLength: 10 });
  const id = sqids.encode([1, 2, 3]);
  assert.equal(id, "86Rf07xd4z");
  const numbers = sqids.decode(id);
  assert.deepEqual(numbers, [1, 2, 3]);
});

test("alphabet option", () => {
  const sqids = new Sqids({
    alphabet: "FxnXM1kBN6cuhsAvjW3Co7l2RePyY8DwaU04Tzt9fHQrqSVKdpimLGIJOgb5ZE",
  });
  const id = sqids.encode([1, 2, 3]);
  assert.equal(id, "B4aajs");
  const numbers = sqids.decode(id);
  assert.deepEqual(numbers, [1, 2, 3]);
});

test("blocklist option", () => {
  const sqids = new Sqids({
    blocklist: new Set(["86Rf07"]),
  });
  const id = sqids.encode([1, 2, 3]);
  assert.equal(id, "se8ojk");
  const numbers = sqids.decode(id);
  assert.deepEqual(numbers, [1, 2, 3]);
});

test("encode empty array", () => {
  const sqids = new Sqids();
  assert.equal(sqids.encode([]), "");
  assert.deepEqual(sqids.decode(""), []);
});

test("invalid numbers throw error", () => {
  const sqids = new Sqids();
  assert.throws(
    () => sqids.encode([-1]),
    /Encoding supports numbers between 0 and 9007199254740991/,
  );
});

test("invalid minLength throws error", () => {
  assert.throws(() => new Sqids({ minLength: -1 }), /Minimum length has to be between 0 and 255/);
  assert.throws(() => new Sqids({ minLength: 300 }), /Minimum length has to be between 0 and 255/);
});
