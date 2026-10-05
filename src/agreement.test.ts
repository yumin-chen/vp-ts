import { expect, test } from "vite-plus/test";
import { diffieHellmanSecret } from "../index.js";

test("diffieHellmanSecret computes shared secret", () => {
  const k1 = Buffer.from("12345678");
  const k2 = Buffer.from("87654321");
  const secret = diffieHellmanSecret(k1, k2);
  expect(Buffer.isBuffer(secret)).toBe(true);
  expect(secret.length).toBe(8);
});
