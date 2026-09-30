import { expect, test } from "vite-plus/test";
import { customAlphabet, nanoid, urlAlphabet } from "../index.js";

test("nanoid generates string of default length 21", () => {
  const id = nanoid();
  expect(id).toHaveLength(21);
});

test("customAlphabet generates string with custom size", () => {
  const generator = customAlphabet("abcdef123456", 8);
  expect(generator()).toHaveLength(8);
  expect(generator(4)).toHaveLength(4);
});

test("urlAlphabet has expected value", () => {
  expect(urlAlphabet).toBe(
    "usemodule-aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ1234567890_-",
  );
});
