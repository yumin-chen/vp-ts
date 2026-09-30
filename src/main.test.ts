import { expect, test } from "vite-plus/test";
import { Ksuid, KsuidMs, main } from "./main.ts";

test("main returns string starting with KSUID:", () => {
  expect(main()).toMatch(/^KSUID: /);
});

test("Ksuid imports from main.ts work as expected", () => {
  const ksuid = Ksuid.now();
  expect(ksuid.toString()).toHaveLength(27);
  expect(ksuid.bytes()).toHaveLength(20);
  expect(ksuid.payload()).toHaveLength(16);
});

test("KsuidMs imports from main.ts work as expected", () => {
  const kms = KsuidMs.now();
  expect(kms.bytes()).toHaveLength(20);
  expect(kms.payload()).toHaveLength(15);
});
