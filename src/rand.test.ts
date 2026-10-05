import { expect, test } from "vite-plus/test";
import { randomBytes, randomFillSync, randomInt, randomUuid } from "../artifacts/index.js";

test("randomInt returns integer within min and max", () => {
  const num = randomInt(1, 10);
  expect(num).toBeGreaterThanOrEqual(1);
  expect(num).toBeLessThan(10);
});

test("randomBytes and randomFillSync and randomUuid", () => {
  const bytes = randomBytes(16);
  expect(bytes.length).toBe(16);

  const buf = Buffer.alloc(8);
  randomFillSync(buf);
  expect(buf.some((b) => b !== 0)).toBe(true);

  const uuid = randomUuid();
  expect(uuid).toMatch(/^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i);
});
