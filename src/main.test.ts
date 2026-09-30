import { expect, test } from "vite-plus/test";
import Sqids, { defaultOptions } from "../index.js";
import { main } from "./main.ts";

test("main returns formatted output", () => {
  expect(main()).toBe("Encoded: 86Rf07, Decoded: 1,2,3");
});

test("Sqids encode and decode", () => {
  const sqids = new Sqids();
  const id = sqids.encode([1, 2, 3]);
  expect(id).toBe("86Rf07");
  expect(sqids.decode(id)).toEqual([1, 2, 3]);
});

test("Sqids with options", () => {
  const sqids = new Sqids({ minLength: 10 });
  const id = sqids.encode([1, 2, 3]);
  expect(id).toBe("86Rf07xd4z");
  expect(sqids.decode(id)).toEqual([1, 2, 3]);
});

test("defaultOptions", () => {
  expect(defaultOptions.minLength).toBe(0);
  expect(defaultOptions.alphabet).toHaveLength(62);
});
