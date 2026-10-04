import assert from "node:assert/strict";
import { createRequire } from "node:module";
import { describe, test } from "node:test";

const require = createRequire(import.meta.url);
const crypto = require("../index.js");

describe("RSA encryption/decryption", () => {
  const input = "I AM THE WALRUS";
  const bufferToEncrypt = Buffer.from(input);

  const { publicKey: rsaPubPem, privateKey: rsaKeyPem } = crypto.generateKeyPairSync(2048);

  test("publicEncrypt and privateDecrypt with rsaKeyPem", () => {
    const encryptedBuffer = crypto.publicEncrypt(rsaPubPem, bufferToEncrypt);
    const decryptedBuffer = crypto.privateDecrypt(rsaKeyPem, encryptedBuffer);
    assert.equal(decryptedBuffer.toString(), input);
  });

  test("privateEncrypt and publicDecrypt with keyPem", () => {
    const encryptedBuffer = crypto.privateEncrypt(rsaKeyPem, bufferToEncrypt);
    const decryptedBuffer = crypto.publicDecrypt(rsaKeyPem, encryptedBuffer);
    assert.equal(decryptedBuffer.toString(), input);
  });

  test("RSA PKCS1 padding", () => {
    const encryptedBuffer = crypto.publicEncrypt(
      {
        key: rsaPubPem,
        padding: crypto.constants.RSA_PKCS1_PADDING,
      },
      bufferToEncrypt,
    );
    const decryptedBuffer = crypto.privateDecrypt(
      {
        key: rsaKeyPem,
        padding: crypto.constants.RSA_PKCS1_PADDING,
      },
      encryptedBuffer,
    );
    assert.equal(decryptedBuffer.toString(), input);
  });

  test("RSA OAEP padding with SHA-256", () => {
    const encryptedBuffer = crypto.publicEncrypt(
      {
        key: rsaPubPem,
        padding: crypto.constants.RSA_PKCS1_OAEP_PADDING,
        oaepHash: "sha256",
      },
      bufferToEncrypt,
    );
    const decryptedBuffer = crypto.privateDecrypt(
      {
        key: rsaKeyPem,
        padding: crypto.constants.RSA_PKCS1_OAEP_PADDING,
        oaepHash: "sha256",
      },
      encryptedBuffer,
    );
    assert.equal(decryptedBuffer.toString(), input);
  });
});
