import { expect, test } from "vite-plus/test";
import { KeyObject, CryptoKeyPair, X509Certificate } from "../dist/index.js";

test("KeyObject constructor and getters", () => {
  const data = Buffer.from("secret-key-material");
  const keyObj = new KeyObject("secret", data);

  expect(keyObj.type).toBe("secret");
  expect(Buffer.isBuffer(keyObj.export())).toBe(true);
  expect(keyObj.export().toString()).toBe("secret-key-material");
});

test("X509Certificate properties and checks", () => {
  const certData = Buffer.from("dummy-certificate-bytes");
  const cert = new X509Certificate(certData);

  expect(cert.raw).toBeDefined();
  expect(typeof cert.fingerprint).toBe("string");
  expect(typeof cert.fingerprint256).toBe("string");
  expect(cert.checkEmail("test@example.com")).toBe(true);
  expect(cert.checkHost("example.com")).toBe("example.com");
});
