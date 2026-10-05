import { expect, test } from "vite-plus/test";
import { decryptGcm, encryptGcm } from "../index.js";

test("AES-256-GCM encrypt and decrypt roundtrip", () => {
  const key = Buffer.alloc(32, 1);
  const nonce = Buffer.alloc(12, 2);
  const plaintext = Buffer.from("secret message");

  const ciphertext = encryptGcm(key, nonce, plaintext);
  expect(Buffer.isBuffer(ciphertext)).toBe(true);

  const decrypted = decryptGcm(key, nonce, ciphertext);
  expect(decrypted.toString()).toBe("secret message");
});
