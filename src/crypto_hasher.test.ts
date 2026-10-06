import assert from "node:assert/strict";
import test from "node:test";
import pkg from "../index.js";

const { createHash, hash, getHashes } = pkg;

void test("createHash and Hash update/digest", () => {
  const hasher = createHash("sha256");
  hasher.update(Buffer.from("hello world"));
  const digestHex = hasher.digest("hex");
  assert.equal(digestHex, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
});

void test("one-shot hash function", () => {
  const res = hash("sha256", Buffer.from("hello world"), "hex");
  assert.equal(res, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
});

void test("getHashes lists algorithms", () => {
  const hashes = getHashes();
  assert.ok(hashes.includes("sha256"));
  assert.ok(hashes.includes("sha512"));
});
