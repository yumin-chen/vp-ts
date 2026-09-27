import { expect, test } from "vitest";
import { main } from "./main.ts";

test("main returns Hello, world!", () => {
  expect(main()).toBe("Hello, world!");
});
