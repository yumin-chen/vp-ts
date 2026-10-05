import { expect, test } from "vite-plus/test";
import { createCipheriv, createDecipheriv } from "../artifacts/index.js";

test("AES-256-GCM cipher and decipher cycle", () => {
  const key = Buffer.alloc(32, "k");
  const iv = Buffer.alloc(12, "i");

  const cipher = createCipheriv("aes-256-gcm", key, iv);
  cipher.update("hello world");
  const ciphertext = cipher.final();
  const tag = cipher.getAuthTag();

  expect(Buffer.isBuffer(ciphertext)).toBe(true);
  expect(Buffer.isBuffer(tag)).toBe(true);
  expect(tag.length).toBe(16);

  const decipher = createDecipheriv("aes-256-gcm", key, iv);
  decipher.update(ciphertext);
  decipher.setAuthTag(tag);
  const plaintext = decipher.final();

  expect(plaintext.toString("utf8")).toBe("hello world");
});
