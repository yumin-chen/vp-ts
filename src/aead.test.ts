import { expect, test } from "vite-plus/test";
import { encryptAesGcm, decryptAesGcm } from "../dist/index.js";

test("encryptAesGcm and decryptAesGcm roundtrip", () => {
  const key = Buffer.alloc(32, 1);
  const iv = Buffer.alloc(12, 2);
  const plaintext = Buffer.from("Hello AES-256-GCM!");
  const aad = Buffer.from("additional-authenticated-data");

  const { ciphertext, authTag } = encryptAesGcm(key, iv, plaintext, aad);
  expect(Buffer.isBuffer(ciphertext)).toBe(true);
  expect(Buffer.isBuffer(authTag)).toBe(true);

  const decrypted = decryptAesGcm(key, iv, ciphertext, authTag, aad);
  expect(decrypted.toString()).toBe("Hello AES-256-GCM!");
});
