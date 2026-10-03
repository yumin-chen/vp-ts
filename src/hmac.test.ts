import crypto from "node:crypto";
import { expect, test } from "vite-plus/test";
import { createHmac, Hmac } from "../index.js";

test("Hmac class computes correct sha256 digest in hex", () => {
  const secret = "secret-key";
  const data = "hello world";

  const expected = crypto.createHmac("sha256", secret).update(data).digest("hex");

  const hmac = new Hmac("sha256", secret);
  hmac.update(data);
  const actual = hmac.digest("hex");

  expect(actual).toBe(expected);
});

test("createHmac function computes correct sha512 digest in base64", () => {
  const secret = "another-secret";
  const data = "some important payload";

  const expected = crypto.createHmac("sha512", secret).update(data).digest("base64");

  const actual = createHmac("sha512", secret).update(data).digest("base64");

  expect(actual).toBe(expected);
});

test("Hmac digest returns Buffer when no encoding is provided", () => {
  const secret = "key";
  const data = "data";

  const expected = crypto.createHmac("sha256", secret).update(data).digest();

  const actual = createHmac("sha256", secret).update(data).digest();

  expect(Buffer.isBuffer(actual)).toBe(true);
  expect(actual.toString("hex")).toBe(expected.toString("hex"));
});
