import { pbkdf2Sync as nodePbkdf2Sync } from "node:crypto";
import { expect, test } from "vite-plus/test";
import { pbkdf2, pbkdf2Sync } from "../dist/index.js";

test("pbkdf2Sync matches node:crypto", () => {
  const pass = "secret";
  const salt = "salt";
  const iterations = 10000;
  const keylen = 64;
  const digest = "sha512";

  const nativeKey = pbkdf2Sync(pass, salt, iterations, keylen, digest);
  const nodeKey = nodePbkdf2Sync(pass, salt, iterations, keylen, digest);

  expect(nativeKey.toString("hex")).toBe(nodeKey.toString("hex"));
});

test("pbkdf2 async matches node:crypto", async () => {
  const pass = "password123";
  const salt = "random-salt";
  const iterations = 5000;
  const keylen = 32;
  const digest = "sha256";

  const nativeKey = await pbkdf2(pass, salt, iterations, keylen, digest);
  const nodeKey = nodePbkdf2Sync(pass, salt, iterations, keylen, digest);

  expect(nativeKey.toString("hex")).toBe(nodeKey.toString("hex"));
});
