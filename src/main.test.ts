import { expect, test } from "vite-plus/test";
import { main } from "./main.ts";

test("main returns @lib/crypto module", () => {
  expect(main()).toBe("@lib/crypto module");
});
