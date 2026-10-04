import { expect, test } from "vite-plus/test";
import {
  createPublicKey,
  createSecretKey,
  createMac,
  Sign,
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
  const message = "data-to-verify";
  const hmacKey = "my-hmac-secret-key-1234567890"; // non 32 byte key for HMAC

  const signer = new Sign("sha256");
  signer.update(message);
  const sig = signer.sign(hmacKey, "buffer") as Buffer;

  const verifier = createVerify("sha256");
  verifier.update(message);
  const isValid = verifier.verify(hmacKey, sig);
  expect(isValid).toBe(true);
});

test("encapsulate and decapsulate", () => {
  const key = createPublicKey("kem-key");
  const { sharedKey, ciphertext } = encapsulate(key);
  expect(Buffer.isBuffer(sharedKey)).toBe(true);
  expect(Buffer.isBuffer(ciphertext)).toBe(true);

  const sharedDec = decapsulate(key, ciphertext);
  expect(Buffer.isBuffer(sharedDec)).toBe(true);
  expect(sharedDec.toString("hex")).toBe(sharedKey.toString("hex"));
});
