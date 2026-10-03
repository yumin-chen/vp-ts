import { expect, test } from "vite-plus/test";
import { Sign, createSign, sign, KeyObject } from "../dist/index.js";

test("Sign class update and sign", () => {
  const signer = new Sign("sha256");
  signer.update("message to sign");
  const keyObj = new KeyObject("private", Buffer.from("private-key-data"));
  const sig = signer.sign(keyObj, "hex");

  expect(typeof sig).toBe("string");
  expect((sig as string).length).toBeGreaterThan(0);
});

test("createSign helper", () => {
  const signer = createSign("sha512");
  signer.update("data");
  const sig = signer.sign("pem-key", "hex");
  expect(typeof sig).toBe("string");
});

test("sign function helper", () => {
  const sigBuf = sign("sha256", "data", "key-material");
  expect(Buffer.isBuffer(sigBuf)).toBe(true);
  expect(sigBuf.length).toBeGreaterThan(0);
});
