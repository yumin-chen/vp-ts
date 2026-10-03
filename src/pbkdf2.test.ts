import { expect, test } from "vite-plus/test";
import { PBKDF2, pbkdf2Sync } from "../artifacts/index.js";

test("pbkdf2Sync derives correct key length", () => {
  const derived = pbkdf2Sync("password", "salt", 1000, 32, "sha256");
  expect(Buffer.isBuffer(derived)).toBe(true);
  expect(derived.length).toBe(32);
});

test("PBKDF2 class derives key matching pbkdf2Sync", () => {
  const pbkdf2 = new PBKDF2("sha256", 1000);
  const derived1 = pbkdf2.deriveSync("password", "salt", 32);
  const derived2 = pbkdf2Sync("password", "salt", 1000, 32, "sha256");

  expect(derived1.toString("hex")).toBe(derived2.toString("hex"));
});

test("pbkdf2Sync works with SHA512", () => {
  const derived = pbkdf2Sync("secret", "salt123", 500, 64, "sha512");
  expect(derived.length).toBe(64);
});
