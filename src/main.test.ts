import { expect, test } from "vite-plus/test";
import { add, main } from "./main.ts";

test("main returns Hello, world!", () => {
  expect(main()).toBe("Hello, world!");
});

test("add adds two numbers", () => {
  expect(add(2, 3)).toBe(5);
});
