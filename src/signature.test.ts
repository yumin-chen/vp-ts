import assert from "node:assert/strict";
import test from "node:test";
import pkg from "../index.js";

const { generateKeyPairSync, sign, verify, createSign, createVerify } = pkg;

void test("RSA sign and verify", () => {
  const { publicKey, privateKey } = generateKeyPairSync("rsa", { modulusLength: 2048 });
  const data = Buffer.from("hello signature world");

  const signature = sign("sha256", data, privateKey);
  assert.ok(signature.length > 0);

  const isValid = verify("sha256", data, publicKey, signature);
  assert.equal(isValid, true);
});

void test("Sign and Verify stream classes", () => {
  const { publicKey, privateKey } = generateKeyPairSync("rsa", { modulusLength: 2048 });
  const data = Buffer.from("stream signature test");

  const signer = createSign("sha256");
  signer.update(data);
  const sig = signer.sign(privateKey);

  const verifier = createVerify("sha256");
  verifier.update(data);
  const ok = verifier.verify(publicKey, sig);
  assert.equal(ok, true);
});
