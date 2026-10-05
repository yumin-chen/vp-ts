import assert from "node:assert/strict";
import test from "node:test";
import { createHmac } from "../index.js";

test("HMAC sha256 generation", () => {
  const hmac = createHmac("sha256", Buffer.from("secret-key"));
  hmac.update(Buffer.from("hello world"));
  const digest = hmac.digest("hex");
  assert.equal(typeof digest, "string");
  assert.equal((digest as string).length, 64);
});
