import crypto from "node:crypto";
import { expect, test } from "vite-plus/test";
import { pbkdf2, pbkdf2Sync } from "../index.js";

test("pbkdf2Sync produces identical output to node:crypto", () => {
  const password = "my-password";
  const salt = "random-salt";
  const iterations = 1000;
  const keylen = 32;
  const digest = "sha256";

  const expected = crypto.pbkdf2Sync(password, salt, iterations, keylen, digest);
  const actual = pbkdf2Sync(password, salt, iterations, keylen, digest);

  expect(Buffer.isBuffer(actual)).toBe(true);
  expect(actual.toString("hex")).toBe(expected.toString("hex"));
});

test("pbkdf2 async produces identical output to node:crypto", async () => {
  const password = "my-password-512";
  const salt = "random-salt-512";
  const iterations = 2000;
  const keylen = 64;
  const digest = "sha512";

  const expected = crypto.pbkdf2Sync(password, salt, iterations, keylen, digest);
  const actual = await pbkdf2(password, salt, iterations, keylen, digest);

  expect(Buffer.isBuffer(actual)).toBe(true);
  expect(actual.toString("hex")).toBe(expected.toString("hex"));
});
