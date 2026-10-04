import { createHmac as nodeCreateHmac } from "node:crypto";
import { expect, test } from "vite-plus/test";
import { Hmac, createHmac } from "../dist/index.js";

test("HMAC sha256 hex digest matches node:crypto", () => {
  const secret = "a secret";
  const message = "Hello world!";

  const nativeHmac = createHmac("sha256", secret);
  nativeHmac.update(message);
  const nativeDigest = nativeHmac.digest("hex");

  const nodeHmac = nodeCreateHmac("sha256", secret);
  nodeHmac.update(message);
  const nodeDigest = nodeHmac.digest("hex");

  expect(nativeDigest).toBe(nodeDigest);
});

test("HMAC class constructor and update chaining/multiple updates", () => {
  const key = Buffer.from("secret-key");
  const hmac = new Hmac("sha512", key);
  hmac.update("part1 ");
  hmac.update("part2");
  const digest = hmac.digest("hex");

  const nodeHmac = nodeCreateHmac("sha512", key);
  nodeHmac.update("part1 ");
  nodeHmac.update("part2");
  expect(digest).toBe(nodeHmac.digest("hex"));
});

test("HMAC buffer digest output", () => {
  const hmac = createHmac("sha256", "key");
  hmac.update("data");
  const buf = hmac.digest("buffer");

  expect(Buffer.isBuffer(buf)).toBe(true);

  const nodeHmac = nodeCreateHmac("sha256", "key");
  nodeHmac.update("data");
  expect(Buffer.from(buf as Buffer).toString("hex")).toBe(nodeHmac.digest("hex"));
});

test("HMAC base64 digest output", () => {
  const hmac = createHmac("sha256", "key");
  hmac.update("data");
  const b64 = hmac.digest("base64");

  const nodeHmac = nodeCreateHmac("sha256", "key");
  nodeHmac.update("data");
  expect(b64).toBe(nodeHmac.digest("base64"));
});
