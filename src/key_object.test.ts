import { expect, test } from "vite-plus/test";
import {
  X509Certificate,
  createPrivateKey,
  createPublicKey,
  createSecretKey,
  generateKey,
  generateKeySync,
  parsePkcs12,
} from "../artifacts/index.js";

test("KeyObject creation helpers", () => {
  const pub = createPublicKey("public key bytes");
  expect(pub.type).toBe("public");

  const priv = createPrivateKey("private key bytes");
  expect(priv.type).toBe("private");

  const secret = createSecretKey("secret key bytes");
  expect(secret.type).toBe("secret");
});

test("X509Certificate properties and checks", () => {
  const cert = new X509Certificate("dummy cert data");
  expect(cert.subject.length).toBeGreaterThan(0);
  expect(cert.fingerprint256.length).toBeGreaterThan(0);
  expect(cert.checkHost("Subject")).toBe(true);
});

test("parsePkcs12 and generateKeySync", () => {
  const p12 = parsePkcs12("bundle content");
  expect(p12.privateKey).not.toBeNull();
  expect(p12.certificate).not.toBeNull();

  const key = generateKeySync("hmac", 256);
  expect(key.type).toBe("secret");
  expect(key.symmetricKeySize).toBe(32);

  const asyncKey = generateKey("aes", 128);
  expect(asyncKey.symmetricKeySize).toBe(16);
});
