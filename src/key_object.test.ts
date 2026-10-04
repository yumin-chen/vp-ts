import assert from "node:assert/strict";
import test from "node:test";
import pkg from "../index.js";

const { KeyObject } = pkg;

void test("KeyObject class", () => {
  const buf = Buffer.from("secret-key-data");
  const key = new KeyObject("secret", buf);

  assert.equal(key.type, "secret");
  assert.equal(key.export().toString("utf8"), "secret-key-data");
});
