const assert = require("node:assert/strict");
const test = require("node:test");

const { establishConnection } = require("./index.js");

test("Native Diesel ORM binding works in CJS", () => {
  const conn = establishConnection(":memory:");
  conn.setupTables();

  const users = conn.insertUsers([
    { name: "Sean", hairColor: "Black", email: "sean@example.com" },
    { name: "Gordon", email: "gordon@example.com" },
  ]);

  assert.equal(users.length, 2);
  assert.equal(users[0].name, "Sean");

  const loadedUsers = conn.loadUsers();
  assert.equal(loadedUsers.length, 2);

  const post = conn.createPost("Diesel Node", "Fast ORM bindings", loadedUsers[0].id);
  assert.equal(post.title, "Diesel Node");
  assert.equal(post.published, false);

  const published = conn.publishPost(post.id);
  assert.equal(published.published, true);

  const userPosts = conn.postsBelongingToUser(loadedUsers[0].id);
  assert.equal(userPosts.length, 1);
  assert.equal(userPosts[0].title, "Diesel Node");
});
