const assert = require("node:assert/strict");
const test = require("node:test");

const { parse, stringify } = require("./index.js");

void test("parses TOML string into object", () => {
  const obj = parse('title = "TOML Example"\n');
  assert.deepEqual(obj, { title: "TOML Example" });
});

void test("stringifies object into TOML string", () => {
  const str = stringify({ title: "TOML Example" });
  assert.equal(str, 'title = "TOML Example"\n');
});
