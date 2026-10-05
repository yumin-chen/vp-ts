import { expect, test } from "vite-plus/test";
import { randomBytes, randomFillSync, randomInt, randomUUID, randomUUIDv7 } from "../index.js";

test("randomBytes generates buffer of requested length", () => {
  const buf = randomBytes(16);
  expect(Buffer.isBuffer(buf)).toBe(true);
  expect(buf.length).toBe(16);
});

test("randomFillSync fills provided buffer", () => {
  const buf = Buffer.alloc(10);
  randomFillSync(buf, 2, 5);
  expect(buf.length).toBe(10);
});

test("randomInt generates number in bounds", () => {
  const num = randomInt(10, 20);
  expect(num).toBeGreaterThanOrEqual(10);
  expect(num).toBeLessThan(20);
});

test("randomUUID and randomUUIDv7 return string UUIDs", () => {
  const uuid4 = randomUUID();
  const uuid7 = randomUUIDv7();
  expect(typeof uuid4).toBe("string");
  expect(typeof uuid7).toBe("string");
  expect(uuid4.length).toBe(36);
  expect(uuid7.length).toBe(36);
});
