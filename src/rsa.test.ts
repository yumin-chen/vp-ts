import { expect, test } from "vite-plus/test";
import { generateKeyPairSync, publicEncrypt, privateDecrypt } from "../index.js";

test("generateKeyPairSync and RSA encryption/decryption", { timeout: 30000 }, () => {
  const { publicKey, privateKey } = generateKeyPairSync("rsa", { modulusLength: 1024 });

  expect(publicKey).toContain("BEGIN PUBLIC KEY");
  expect(privateKey).toContain("BEGIN PRIVATE KEY");

  const message = Buffer.from("Secret RSA message");
  const encrypted = publicEncrypt(publicKey, message);
  expect(Buffer.isBuffer(encrypted)).toBe(true);

  const decrypted = privateDecrypt(privateKey, encrypted);
  expect(decrypted.toString()).toBe("Secret RSA message");
});
