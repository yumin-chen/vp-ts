import { expect, test } from "vite-plus/test";
import { main } from "./main.ts";

test("main returns Hello, world! string with addition result", () => {
  expect(main()).toBe("Hello, world! 2 + 3 = 5");
});
