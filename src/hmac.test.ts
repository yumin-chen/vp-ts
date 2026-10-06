import { describe, expect, it } from "vite-plus/test";
import { Hmac, createHmac } from "../index.js";

describe("HMAC", () => {
  it("should compute HMAC SHA256 hex digest correctly", () => {
    const hmac = new Hmac("sha256", "secret-key");
    hmac.update("hello world");
    const hex = hmac.digest("hex");
    expect(typeof hex).toBe("string");
    expect(hex).toHaveLength(64);
  });

  it("should compute HMAC SHA256 Buffer digest correctly", () => {
    const hmac = createHmac("sha256", Buffer.from("secret-key"));
    hmac.update(Buffer.from("hello world"));
    const buf = hmac.digest();
    expect(Buffer.isBuffer(buf)).toBe(true);
    expect(buf.length).toBe(32);
  });
});
