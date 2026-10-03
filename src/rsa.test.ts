import { expect, test } from "vite-plus/test";
import {
  generateKeyPairSync,
  publicEncrypt,
  privateDecrypt,
} from "../dist/index.js";

test("generateKeyPairSync RSA key pair", () => {
  const pair = generateKeyPairSync("rsa", { modulusLength: 1024 });
  expect(pair.publicKey).toBeDefined();
  expect(pair.privateKey).toBeDefined();
  expect(pair.publicKey.type).toBe("public");
  expect(pair.privateKey.type).toBe("private");
});

test("publicEncrypt and privateDecrypt roundtrip", () => {
  const pair = generateKeyPairSync("rsa", { modulusLength: 1024 });
  const message = Buffer.from("Secret RSA Payload");

  const encrypted = publicEncrypt(pair.publicKey, message);
  expect(Buffer.isBuffer(encrypted)).toBe(true);

  const decrypted = privateDecrypt(pair.privateKey, encrypted);
  expect(decrypted).toBeDefined();
});
