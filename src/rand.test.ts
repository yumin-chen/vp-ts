import { expect, test } from "vite-plus/test";
import {
  randomBytes,
  randomFillSync,
  randomInt,
  randomUUID,
  randomUUIDv7,
} from "../dist/index.js";

test("randomBytes generates buffer of requested length", () => {
  const buf = randomBytes(16);
  expect(Buffer.isBuffer(buf)).toBe(true);
  expect(buf.length).toBe(16);
});

test("randomFillSync fills Uint8Array", () => {
  const arr = new Uint8Array(10);
  randomFillSync(arr, 0, 10);
  expect(arr.some((val) => val !== 0)).toBe(true);
});

test("randomInt generates number within bounds", () => {
  const num = randomInt(5, 15);
  expect(num).toBeGreaterThanOrEqual(5);
  expect(num).toBeLessThan(15);
});

test("randomUUID generates valid v4 and v7 UUID strings", () => {
  const uuid4 = randomUUID();
  expect(typeof uuid4).toBe("string");
  expect(uuid4.length).toBe(36);

  const uuid7 = randomUUIDv7();
  expect(typeof uuid7).toBe("string");
  expect(uuid7.length).toBe(36);
});
