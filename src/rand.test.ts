import assert from "node:assert/strict";
import test from "node:test";
import pkg from "../index.js";

const { randomBytes, randomFillSync, randomInt, randomUuid, randomUuidv7, getRandomValues } = pkg;

void test("randomBytes", () => {
  const buf = randomBytes(16);
  assert.equal(buf.length, 16);
});

void test("randomFillSync", () => {
  const buf = Buffer.alloc(10);
  randomFillSync(buf, 2, 5);
  assert.equal(buf.length, 10);
});

void test("randomInt", () => {
  const num = randomInt(1, 100);
  assert.ok(num >= 1 && num < 100);
});

void test("randomUuid & v7", () => {
  const uuid4 = randomUuid();
  const uuid7 = randomUuidv7();
  assert.equal(uuid4.length, 36);
  assert.equal(uuid7.length, 36);
});

void test("getRandomValues", () => {
  const buf = Buffer.alloc(16);
  getRandomValues(buf);
  assert.equal(buf.length, 16);
});
