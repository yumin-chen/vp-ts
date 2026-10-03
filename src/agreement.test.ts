import { expect, test } from "vite-plus/test";
import {
  createPublicKey,
  createSecretKey,
  createMac,
  Verify,
  createVerify,
  encapsulate,
  decapsulate,
} from "../dist/index.js";

test("createPublicKey and createSecretKey", () => {
  const pub = createPublicKey("public-key-bytes");
  expect(pub.type).toBe("public");

  const secret = createSecretKey("secret-bytes");
  expect(secret.type).toBe("secret");

  const macKey = createMac("hmac", "mac-key");
  expect(macKey.type).toBe("secret");
});

test("Verify class and createVerify", () => {
  const verifier = createVerify("sha256");
  verifier.update("data");
  const isValid = verifier.verify("public-key", Buffer.from("sig"));
  expect(isValid).toBe(true);
});

test("encapsulate and decapsulate", () => {
  const key = createPublicKey("kem-key");
  const { sharedKey, ciphertext } = encapsulate(key);
  expect(Buffer.isBuffer(sharedKey)).toBe(true);
  expect(Buffer.isBuffer(ciphertext)).toBe(true);

  const sharedDec = decapsulate(key, ciphertext);
  expect(Buffer.isBuffer(sharedDec)).toBe(true);
});
