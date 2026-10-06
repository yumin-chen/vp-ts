import assert from "node:assert/strict";
import test from "node:test";
import pkg from "../index.js";

const { encryptAead, decryptAead, getCiphers, getCipherInfo } = pkg;

void test("AEAD aes-256-gcm encrypt and decrypt", () => {
  const key = Buffer.alloc(32, 0x01);
  const iv = Buffer.alloc(12, 0x02);
  const plaintext = Buffer.from("Top secret AEAD message");
  const aad = Buffer.from("additional authenticated data");

  const res = encryptAead("aes-256-gcm", key, iv, plaintext, aad);
  assert.ok(res.ciphertext.length > 0);
  assert.equal(res.tag.length, 16);

  const decrypted = decryptAead("aes-256-gcm", key, iv, res.ciphertext, res.tag, aad);
  assert.equal(decrypted.toString("utf8"), "Top secret AEAD message");
});

void test("getCiphers and getCipherInfo", () => {
  const ciphers = getCiphers();
  assert.ok(ciphers.includes("aes-256-gcm"));

  const info = getCipherInfo("aes-256-gcm");
  assert.ok(info);
  assert.equal(info?.keyLength, 32);
  assert.equal(info?.mode, "gcm");
});
