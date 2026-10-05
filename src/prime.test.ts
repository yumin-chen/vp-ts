import { expect, test } from "vite-plus/test";
import { checkPrimeSync, generatePrimeSync } from "../index.js";

test("generatePrimeSync and checkPrimeSync", () => {
  const primeBuf = generatePrimeSync(64);
  expect(Buffer.isBuffer(primeBuf)).toBe(true);
  expect(primeBuf.length).toBe(8);

  const isPrime = checkPrimeSync(primeBuf);
  expect(typeof isPrime).toBe("boolean");
});
