import { expect, test } from "vite-plus/test";
import { createEcdh } from "../index.js";

test("ECDH key generation", () => {
  const ecdh = createEcdh("x25519");
  const pub = ecdh.getPublicKey();
  expect(Buffer.isBuffer(pub)).toBe(true);
  expect(pub.length).toBe(32);
});
