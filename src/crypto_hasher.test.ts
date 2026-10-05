import { expect, test } from "vite-plus/test";
import { createHash, hash, getHashes } from "../index.js";

test("getHashes returns list of algorithms", () => {
  const hashes = getHashes();
  expect(hashes).toContain("sha256");
  expect(hashes).toContain("md5");
});

test("createHash and digest hex", () => {
  const h = createHash("sha256");
  h.update("hello");
  const digest = h.digest("hex");
  expect(digest).toBe("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
});

test("hash one shot", () => {
  const digest = hash("sha256", "hello", "hex");
  expect(digest).toBe("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
});
