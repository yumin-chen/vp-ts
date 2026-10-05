import { expect, test } from "vite-plus/test";
import { generateKeyPairSync } from "../artifacts/index.js";

test("generateKeyPairSync generates RSA key pair", () => {
  const pair = generateKeyPairSync("rsa", 1024);
  expect(pair.publicKey.type).toBe("public");
  expect(pair.privateKey.type).toBe("private");

  const pubPem = pair.publicKey.export().toString("utf8");
  expect(pubPem).toContain("PUBLIC KEY");
}, 30000);
