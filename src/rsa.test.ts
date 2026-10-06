import { describe, expect, test } from "vite-plus/test";
import { privateDecrypt, privateEncrypt, publicDecrypt, publicEncrypt } from "./main.js";

describe("RSA encryption/decryption", () => {
  test("exports RSA functions", () => {
    expect(typeof publicEncrypt).toBe("function");
    expect(typeof privateDecrypt).toBe("function");
    expect(typeof publicDecrypt).toBe("function");
    expect(typeof privateEncrypt).toBe("function");
  });
});
