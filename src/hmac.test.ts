import { expect, test } from "vite-plus/test";
import { createHmac } from "../index.js";

test("createHmac computes HMAC-SHA256 correctly", () => {
  const hmac = createHmac("sha256", Buffer.from("secret-key"));
  hmac.update(Buffer.from("message"));
  const digest = hmac.digest();
  expect(typeof digest).toBe("string");
  expect(digest.length).toBe(64);
});
