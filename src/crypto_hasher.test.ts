import { expect, test } from "vite-plus/test";
import { createHash } from "../index.js";

test("createHash computes SHA-256 digest correctly", () => {
  const hasher = createHash("sha256");
  hasher.update(Buffer.from("hello world"));
  const digest = hasher.digest("hex");
  expect(digest).toBe("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
});
