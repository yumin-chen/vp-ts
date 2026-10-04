import { expect, test } from "vite-plus/test";
import { createHmac, Hmac } from "../index.js";

test("Hmac computes sha256 hex digest correctly", () => {
  const hmac = createHmac("sha256", "secret");
  hmac.update("hello world");
  const digest = hmac.digest("hex");
  expect(typeof digest).toBe("string");
  expect(digest).toBe("734cc62f32841568f45715aeb9f4d7891324e6d948e4c6c60c0621cdac48623a");
});

test("Hmac class instantiation", () => {
  const hmac = new Hmac("sha256", Buffer.from("key"));
  hmac.update(Buffer.from("data"));
  const buf = hmac.digest();
  expect(Buffer.isBuffer(buf)).toBe(true);
});
