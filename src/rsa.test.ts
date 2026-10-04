import assert from "node:assert/strict";
import test from "node:test";
import pkg from "../index.js";

const { generateKeyPairSync, generateKeyPair, publicEncrypt, privateDecrypt } = pkg;

void test("RSA generateKeyPairSync, publicEncrypt and privateDecrypt", () => {
  const { publicKey, privateKey } = generateKeyPairSync("rsa", { modulusLength: 2048 });
  assert.ok(publicKey.includes("BEGIN PUBLIC KEY"));
  assert.ok(privateKey.includes("BEGIN PRIVATE KEY"));

  const msg = Buffer.from("hello rsa secret");
  const encrypted = publicEncrypt(publicKey, msg);
  assert.ok(encrypted.length > 0);

  const decrypted = privateDecrypt(privateKey, encrypted);
  assert.equal(decrypted.toString("utf8"), "hello rsa secret");
});

void test("RSA async generateKeyPair", async () => {
  const pair = await new Promise<{ publicKey: string; privateKey: string }>((resolve, reject) => {
    generateKeyPair(
      "rsa",
      { modulusLength: 2048 },
      (err: Error | null, pub?: string, priv?: string) => {
        if (err) reject(err);
        else resolve({ publicKey: pub!, privateKey: priv! });
      },
    );
  });
  assert.ok(pair.publicKey);
  assert.ok(pair.privateKey);
});
