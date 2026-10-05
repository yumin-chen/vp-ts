import { expect, test } from "vite-plus/test";
import { pbkdf2Sync } from "../index.js";

test("pbkdf2Sync derives key correctly", () => {
  const derived = pbkdf2Sync(Buffer.from("password"), Buffer.from("salt"), 1000, 32);
  expect(Buffer.isBuffer(derived)).toBe(true);
  expect(derived.length).toBe(32);
});
