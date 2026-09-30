const assert = require("node:assert/strict");
const test = require("node:test");

void test("DieselOrm native binding in CommonJS", async () => {
  const { DieselOrm } = await import("./index.js");
  assert.ok(DieselOrm);
  const db = new DieselOrm(":memory:");
  assert.ok(db);
  const users = db.loadAllUsers();
  assert.equal(Array.isArray(users), true);
  assert.equal(users.length, 0);
});
