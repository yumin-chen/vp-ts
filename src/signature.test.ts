import { expect, test } from "vite-plus/test";
import { createSign, createVerify, generateKeyPairSync } from "../artifacts/index.js";

test("Sign and Verify cycle", () => {
  const pair = generateKeyPairSync("rsa", 1024);
  const privPem = pair.privateKey.export().toString("utf8");
  const pubPem = pair.publicKey.export().toString("utf8");

  const signer = createSign("SHA256");
  signer.update("message to sign");
  const sig = signer.sign(privPem, "hex");
  expect(typeof sig).toBe("string");

  const verifier = createVerify("SHA256");
  verifier.update("message to sign");
  expect(verifier.verify(pubPem, Buffer.from(sig as string, "hex"))).toBe(true);
});
