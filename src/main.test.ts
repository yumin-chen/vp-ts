import { expect, test } from "vite-plus/test";
import { generateKsuid, getKsuidTime } from "./main.ts";

test("generateKsuid returns a 27-character KSUID", () => {
  const ksuid = generateKsuid();
  expect(ksuid).toHaveLength(27);
  expect(getKsuidTime(ksuid)).toBeGreaterThan(1600000000);
});
