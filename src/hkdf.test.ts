import assert from "node:assert/strict";
import test from "node:test";
import pkg from "../index.js";

const { hkdfSync, hkdf } = pkg;

void test("hkdfSync", () => {
  const ikm = Buffer.from("ikm");
  const salt = Buffer.from("salt");
  const info = Buffer.from("info");
  const res = hkdfSync("sha256", ikm, salt, info, 32);
  assert.equal(res.length, 32);
});

void test("hkdf async", async () => {
  const ikm = Buffer.from("ikm");
  const salt = Buffer.from("salt");
  const info = Buffer.from("info");
  const res = await hkdf("sha256", ikm, salt, info, 32);
  assert.equal(res.length, 32);
});
