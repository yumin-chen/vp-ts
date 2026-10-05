import { expect, test } from "vite-plus/test";
import { generateKeyPairSync, createSign, createVerify, sign, verify } from "../index.js";

test("Sign and Verify classes", { timeout: 30000 }, () => {
  const { publicKey, privateKey } = generateKeyPairSync("rsa", { modulusLength: 1024 });

  const signer = createSign("SHA256");
  signer.update("message to sign");
  const signature = signer.sign(privateKey);

  expect(Buffer.isBuffer(signature)).toBe(true);

  const verifier = createVerify("SHA256");
  verifier.update("message to sign");
  const isValid = verifier.verify(publicKey, signature);

  expect(isValid).toBe(true);
});

test("sign and verify one-shot functions", { timeout: 30000 }, () => {
  const { publicKey, privateKey } = generateKeyPairSync("rsa", { modulusLength: 1024 });

  const signature = sign("SHA256", "hello world", privateKey);
  const isValid = verify("SHA256", "hello world", publicKey, signature);

  expect(isValid).toBe(true);
});
