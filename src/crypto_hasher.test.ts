import { describe, expect, it } from "vite-plus/test";
import { CryptoHasher, getHashes, hash } from "../index.js";

describe("CryptoHasher", () => {
  it("should compute SHA256 digest", () => {
    const hasher = new CryptoHasher("sha256");
    hasher.update("hello world");
    const hex = hasher.digest("hex");
    expect(hex).toBe("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
  });

  it("should compute one-shot hash", () => {
    const hex = hash("sha256", "hello world", "hex");
    expect(hex).toBe("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
  });

  it("should list available hashes", () => {
    const hashes = getHashes();
    expect(hashes).toContain("sha256");
    expect(hashes).toContain("sha512");
  });
});
