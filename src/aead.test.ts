import { expect, test } from "vite-plus/test";
import { createCipheriv, createDecipheriv } from "../index.js";

test("AES-256-GCM encryption and decryption", () => {
  const key = Buffer.alloc(32, 1);
  const iv = Buffer.alloc(12, 2);

  const cipher = createCipheriv("aes-256-gcm", key, iv);
  cipher.setAad(Buffer.from("aad"));
  cipher.update("hello world");
  const encrypted = cipher.final();
  const authTag = cipher.getAuthTag();

  expect(Buffer.isBuffer(encrypted)).toBe(true);
  expect(Buffer.isBuffer(authTag)).toBe(true);

  const decipher = createDecipheriv("aes-256-gcm", key, iv);
  decipher.setAad(Buffer.from("aad"));
  decipher.setAuthTag(authTag);
  decipher.update(encrypted);
  const decrypted = decipher.final();

  expect(decrypted.toString()).toBe("hello world");
});
