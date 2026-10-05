import { expect, test } from "vite-plus/test";
import {
  createSign,
  createVerify,
  decapsulate,
  encapsulate,
  generateKeyPairSync,
} from "../artifacts/index.js";

test("encapsulate and decapsulate", () => {
  const enc = encapsulate("public-key");
  expect(Buffer.isBuffer(enc.sharedKey)).toBe(true);
  expect(Buffer.isBuffer(enc.ciphertext)).toBe(true);

  const shared = decapsulate("private-key", enc.ciphertext);
  expect(Buffer.isBuffer(shared)).toBe(true);
});

test("Verify class and verify function", () => {
  const pair = generateKeyPairSync("rsa", 1024);
  const privPem = pair.privateKey.export().toString("utf8");
  const pubPem = pair.publicKey.export().toString("utf8");

  const signer = createSign("SHA256");
  signer.update("data");
  const sig = signer.sign(privPem);

  const verifier = createVerify("SHA256");
  verifier.update("data");
  expect(verifier.verify(pubPem, sig)).toBe(true);
});
