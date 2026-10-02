const assert = require("node:assert/strict");
const test = require("node:test");

void test("creates and interacts with Container", async () => {
  const { Container, ContainerCompose, ContainerBuild } = await import("./index.js");
  assert.ok(Container);
  assert.ok(ContainerCompose);
  assert.ok(ContainerBuild);
  const container = Container.withDefaultConfig();
  assert.ok(container);
  const createdContainer = container.create({ image: "alpine" }, "my-test");
  assert.ok(createdContainer);
  const info = container.getInfo("my-test");
  assert.equal(info.id, "my-test");
});
