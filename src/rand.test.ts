import { expect, test } from "vite-plus/test";
import { randomBytes, randomFillSync, randomInt, randomUuid } from "../index.js";

test("rand module utilities function properly", () => {
  const bytes = randomBytes(16);
  expect(bytes.length).toBe(16);

  const filled = randomFillSync(Buffer.alloc(8));
  expect(filled.length).toBe(8);

  const num = randomInt(1, 10);
  expect(num).toBeGreaterThanOrEqual(1);
  expect(num).toBeLessThan(10);

  const uuid = randomUuid();
  expect(typeof uuid).toBe("string");
  expect(uuid.length).toBe(36);
});
