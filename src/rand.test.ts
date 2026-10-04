import assert from "node:assert/strict";
import test from "node:test";
import { randomBytes, randomInt, randomUuid, randomUuidv7 } from "../index.js";

test("randomBytes generation", () => {
  const bytes = randomBytes(16);
  assert.equal(bytes.length, 16);
});

test("randomInt range", () => {
  const val = randomInt(10, 20);
  assert.ok(val >= 10 && val < 20);
});

test("randomUuid and v7", () => {
  const u4 = randomUuid();
  assert.equal(u4.length, 36);

  const u7 = randomUuidv7();
  assert.equal(u7.length, 36);
});
