const assert = require("node:assert/strict");
const test = require("node:test");

const { nanoid, customAlphabet, customRandom, urlAlphabet } = require("./index.js");
const {
  nanoid: nonSecureNanoid,
  customAlphabet: nonSecureCustomAlphabet,
  urlAlphabet: nonSecureUrlAlphabet,
} = require("./non-secure.js");

test("urlAlphabet has expected string", () => {
  assert.equal(
    urlAlphabet,
    "usemodule-aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ1234567890_-",
  );
  assert.equal(nonSecureUrlAlphabet, urlAlphabet);
});

test("nanoid generates ID with default length 21", () => {
  const id = nanoid();
  assert.equal(typeof id, "string");
  assert.equal(id.length, 21);
});

test("nanoid generates ID with specified length", () => {
  const id = nanoid(10);
  assert.equal(typeof id, "string");
  assert.equal(id.length, 10);
});

test("customAlphabet creates generator with default and override size", () => {
  const generator = customAlphabet("1234567890abcdef", 10);
  const id1 = generator();
  assert.equal(id1.length, 10);
  assert.match(id1, /^[1234567890abcdef]+$/);

  const id2 = generator(5);
  assert.equal(id2.length, 5);
  assert.match(id2, /^[1234567890abcdef]+$/);
});

test("customAlphabet throws error for invalid alphabet", () => {
  assert.throws(() => customAlphabet(""), /Alphabet must contain from 1 to 256 symbols/);
  assert.throws(
    () => customAlphabet("a".repeat(257)),
    /Alphabet must contain from 1 to 256 symbols/,
  );
});

test("customRandom with seed-based generator produces deterministic ID", () => {
  // Simple LCG PRNG for testing
  let seed = 12345;
  const rng = () => {
    seed = (seed * 1664525 + 1013904223) % 4294967296;
    return seed / 4294967296;
  };

  const myNanoid = customRandom("abcdef", 10, (size) => {
    return new Uint8Array(size).map(() => 256 * rng());
  });

  const id = myNanoid();
  assert.equal(id.length, 10);
  assert.match(id, /^[abcdef]+$/);
});

test("non-secure nanoid generates ID", () => {
  const id = nonSecureNanoid();
  assert.equal(id.length, 21);

  const shortId = nonSecureNanoid(8);
  assert.equal(shortId.length, 8);

  const generator = nonSecureCustomAlphabet("123456", 6);
  const customId = generator();
  assert.equal(customId.length, 6);
  assert.match(customId, /^[123456]+$/);
});
