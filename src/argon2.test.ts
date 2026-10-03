import { describe, expect, it } from "vite-plus/test";
import { argon2, argon2Sync } from "../index.js";

describe("Argon2", () => {
  it("should compute argon2Sync hash", () => {
    const hash = argon2Sync("password", "saltsaltsalt");
    expect(Buffer.isBuffer(hash)).toBe(true);
    expect(hash.length).toBe(32);
  });

  it("should compute async argon2 hash", () => {
    const hash = argon2("password", "saltsaltsalt", { memoryCost: 4096, timeCost: 2 });
    expect(Buffer.isBuffer(hash)).toBe(true);
    expect(hash.length).toBe(32);
  });
});
