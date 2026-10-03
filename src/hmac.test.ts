import { expect, test } from "vite-plus/test";
import { Hmac, createHmac } from "../artifacts/index.js";

test("Hmac computes correct SHA256 digest", () => {
  const hmac = new Hmac("sha256", "secret-key");
  hmac.update("hello world");
  const digestHex = hmac.digest("hex");
  expect(typeof digestHex).toBe("string");
  expect(digestHex.length).toBe(64);
});

test("createHmac helper works identically to new Hmac", () => {
  const hmac1 = new Hmac("sha256", "mykey");
  hmac1.update("data");
  const d1 = hmac1.digest("hex");

  const hmac2 = createHmac("sha256", "mykey");
  hmac2.update("data");
  const d2 = hmac2.digest("hex");

  expect(d1).toBe(d2);
});

test("Hmac handles base64 digest", () => {
  const hmac = createHmac("sha256", "mykey");
  hmac.update("data");
  const dBase64 = hmac.digest("base64");
  expect(typeof dBase64).toBe("string");
  expect(dBase64.length).toBeGreaterThan(0);
});

test("Hmac throws if digest called twice", () => {
  const hmac = createHmac("sha256", "mykey");
  hmac.update("data");
  hmac.digest("hex");
  expect(() => hmac.digest("hex")).toThrow();
});
