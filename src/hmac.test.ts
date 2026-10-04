import assert from "node:assert/strict";
import test from "node:test";
import pkg from "../index.js";

const { createHmac, getMacs } = pkg;

void test("createHmac sha256", () => {
  const hmac = createHmac("sha256", Buffer.from("key"));
  hmac.update(Buffer.from("The quick brown fox jumps over the lazy dog"));
  const digest = hmac.digest("hex");
  assert.equal(typeof digest, "string");
  assert.equal(digest.length, 64);
});

void test("getMacs", () => {
  const macs = getMacs();
  assert.ok(macs.includes("hmac"));
});
