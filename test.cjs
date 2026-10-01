const assert = require("node:assert/strict");
const test = require("node:test");

void test("JsContainer works via index.js native binding", async () => {
  const { JsContainer } = await import("./index.js");
  const container = JsContainer.withDefaultConfig();
  assert.ok(container);

  const handle = await container.create({ image: "alpine:latest" }, "cjs-test");
  assert.ok(handle);

  const execRes = await handle.exec("echo", ["cjs"]);
  assert.equal(execRes, "Executed: echo cjs");
});
