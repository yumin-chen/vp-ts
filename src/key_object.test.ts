import { expect, test } from "vite-plus/test";
import { KeyObject, CryptoKeyPair } from "../index.js";

test("KeyObject properties and methods", () => {
  const raw = Buffer.from("secret key material");
  const key = new KeyObject("secret", raw, null);

  expect(key.type).toBe("secret");
  expect(key.export().toString()).toBe("secret key material");
});

test("CryptoKeyPair getter properties", () => {
  const pub = Buffer.from("public data");
  const priv = Buffer.from("private data");
  const pair = new CryptoKeyPair(pub, priv);

  expect(pair.publicKey.type).toBe("public");
  expect(pair.privateKey.type).toBe("private");
});
