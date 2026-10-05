import { expect, test } from "vite-plus/test";
import { generateKeyPairSync } from "../index.js";

test("RSA generateKeyPairSync generates PEM pair", { timeout: 15000 }, () => {
  const pair = generateKeyPairSync(1024);
  expect(pair.publicKey).toContain("PUBLIC KEY");
  expect(pair.privateKey).toContain("PRIVATE KEY");
});
