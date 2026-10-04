import { expect, test } from "vite-plus/test";
import { createSign } from "../index.js";

test("Signer updates and produces digest signature", () => {
  const signer = createSign("sha256");
  signer.update(Buffer.from("hello signature"));
  const sig = signer.sign(Buffer.from("key"));
  expect(Buffer.isBuffer(sig)).toBe(true);
  expect(sig.length).toBe(32);
});
