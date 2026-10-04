import { expect, test } from "vite-plus/test";
import {
  createPublicKey,
  createPrivateKey,
  createSecretKey,
  encapsulate,
  decapsulate,
  diffieHellman,
} from "../index.js";

test("Key object creation functions", () => {
  const pub = createPublicKey("public key pem");
  expect(pub.type).toBe("public");

  const priv = createPrivateKey("private key pem");
  expect(priv.type).toBe("private");

  const sec = createSecretKey("secret key");
  expect(sec.type).toBe("secret");
});

test("encapsulate and decapsulate", () => {
  const res = encapsulate(Buffer.from("key"));
  expect(Buffer.isBuffer(res.sharedKey)).toBe(true);
  expect(Buffer.isBuffer(res.ciphertext)).toBe(true);

  const shared = decapsulate(Buffer.from("key"), res.ciphertext);
  expect(Buffer.isBuffer(shared)).toBe(true);
});

test("diffieHellman function", () => {
  const priv = Buffer.from([1, 2, 3, 4]);
  const pub = Buffer.from([5, 6, 7, 8]);
  const secret = diffieHellman(priv, pub);
  expect(secret.length).toBe(4);
});
