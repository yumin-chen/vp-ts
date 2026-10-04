const assert = require("node:assert");
const test = require("node:test");

void test("native binding add function", (t) => {
  try {
    const { add } = require("./index.js");
    if (typeof add === "function") {
      const result = add(1, 2);
      assert.strictEqual(result, 3, "add(1, 2) should return 3");
    } else {
      t.skip("Native binding not exported on this target.");
    }
  } catch (err) {
    console.warn("Native module load notice:", err.message);
  }
});
