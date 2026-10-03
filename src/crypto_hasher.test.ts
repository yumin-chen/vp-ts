import crypto from "node:crypto";
import { expect, test } from "vite-plus/test";
import { createHash, getHashes, hash, Hash } from "../index.js";

test("Hash class computes correct sha256 hex digest", () => {
  const data = "hello world from crypto_hasher";
  const expected = crypto.createHash("sha256").update(data).digest("hex");

  const hasher = new Hash("sha256");
  hasher.update(data);
  const actual = hasher.digest("hex");

  expect(actual).toBe(expected);
});

test("createHash function computes correct sha512 base64 digest", () => {
  const data = "another test string";
  const expected = crypto.createHash("sha512").update(data).digest("base64");

  const actual = createHash("sha512").update(data).digest("base64");

  expect(actual).toBe(expected);
});

test("hash one-shot function produces identical output to node:crypto", () => {
  const data = "one-shot hash test";
  const expected = crypto.hash("sha256", data, "hex");
  const actual = hash("sha256", data, "hex");

  expect(actual).toBe(expected);
});

test("getHashes returns supported digest algorithms", () => {
  const hashes = getHashes();
  expect(hashes).toContain("sha1");
  expect(hashes).toContain("sha256");
  expect(hashes).toContain("sha384");
  expect(hashes).toContain("sha512");
});
