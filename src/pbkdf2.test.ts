import { describe, expect, it } from "vite-plus/test";
import { pbkdf2, pbkdf2Sync } from "../index.js";

describe("PBKDF2", () => {
  it("should compute pbkdf2Sync derived key", () => {
    const derivedKey = pbkdf2Sync("secret", "salt", 1000, 32, "sha256");
    expect(Buffer.isBuffer(derivedKey)).toBe(true);
    expect(derivedKey.length).toBe(32);
  });

  it("should compute pbkdf2 derived key", () => {
    const derivedKey = pbkdf2("secret", "salt", 1000, 64, "sha512");
    expect(Buffer.isBuffer(derivedKey)).toBe(true);
    expect(derivedKey.length).toBe(64);
  });
});
