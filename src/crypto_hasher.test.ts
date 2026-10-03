import { createHash as nodeCreateHash } from "node:crypto";
import { expect, test } from "vite-plus/test";
import { CryptoHasher, createHash, cryptoHash } from "../dist/index.js";

test("CryptoHasher sha256 matches node:crypto createHash", () => {
  const data = "Hello, world!";
  const hasher = createHash("sha256");
  hasher.update(data);
  const nativeDigest = hasher.digest("hex");

  const nodeHasher = nodeCreateHash("sha256");
  nodeHasher.update(data);
  const nodeDigest = nodeHasher.digest("hex");

  expect(nativeDigest).toBe(nodeDigest);
});

test("CryptoHasher class constructor and sha512", () => {
  const data = "Testing CryptoHasher";
  const hasher = new CryptoHasher("sha512");
  hasher.update(data);
  const nativeDigest = hasher.digest("hex");

  const nodeHasher = nodeCreateHash("sha512");
  nodeHasher.update(data);
  expect(nativeDigest).toBe(nodeHasher.digest("hex"));
});

test("cryptoHash helper function", () => {
  const data = "One shot hash";
  const nativeDigest = cryptoHash("sha256", data, "hex");

  const nodeHasher = nodeCreateHash("sha256");
  nodeHasher.update(data);
  expect(nativeDigest).toBe(nodeHasher.digest("hex"));
});
