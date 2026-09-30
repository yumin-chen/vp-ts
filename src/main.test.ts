import { expect, test } from "vite-plus/test";
import { parse, stringify } from "../index.js";
import { main } from "./main.ts";

test("main returns formatted string", () => {
  expect(main()).toBe('Parsed key: value, Stringified: key = "value"');
});

test("parse and stringify", () => {
  const obj = parse('title = "TOML Example"\n');
  expect(obj).toEqual({ title: "TOML Example" });
  const str = stringify(obj);
  expect(str).toBe('title = "TOML Example"\n');
});
