import { describe, expect, it } from "vite-plus/test";
import { aeadDecrypt, aeadEncrypt } from "../index.js";

describe("AEAD", () => {
  it("should encrypt and decrypt using aes-128-gcm", () => {
    const key = Buffer.alloc(16, "k");
    const iv = Buffer.alloc(12, "i");
    const plaintext = Buffer.from("hello aead gcm");

    const res = aeadEncrypt("aes-128-gcm", key, iv, plaintext);
    expect(res.ciphertext.length).toBe(plaintext.length);
    expect(res.tag.length).toBe(16);

    const decrypted = aeadDecrypt("aes-128-gcm", key, iv, res.ciphertext, res.tag);
    expect(decrypted.toString()).toBe("hello aead gcm");
  });

  it("should encrypt and decrypt using aes-256-siv", () => {
    const key = Buffer.alloc(64, "k");
    const iv = Buffer.alloc(16, "i");
    const plaintext = Buffer.from("hello aead siv");

    const res = aeadEncrypt("aes-256-siv", key, iv, plaintext);
    expect(res.ciphertext.length).toBe(plaintext.length);
    expect(res.tag.length).toBe(16);

    const decrypted = aeadDecrypt("aes-256-siv", key, iv, res.ciphertext, res.tag);
    expect(decrypted.toString()).toBe("hello aead siv");
  });
});
