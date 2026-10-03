import { expect, test } from "vite-plus/test";
import { add } from "../index.js";

test("lib.rs add function adds two numbers correctly", () => {
  expect(add(2, 3)).toBe(5);
  expect(add(-1, 1)).toBe(0);
});
