import assert from "node:assert/strict";
import test from "node:test";
import { createHash, getHashes } from "../index.js";

test("Hash SHA256", () => {
  const hash = createHash("sha256");
  hash.update(Buffer.from("test string"));
  const res = hash.digest("hex");
  assert.equal(typeof res, "string");
  assert.equal((res as string).length, 64);
});

test("getHashes returns list of algorithms", () => {
  const hashes = getHashes();
  assert.ok(hashes.includes("sha256"));
});
