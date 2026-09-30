import assert from "node:assert/strict";
import test from "node:test";
import {
  createPost,
  deletePost,
  establishConnection,
  getPost,
  PostModel,
  publishPost,
  showPosts,
  users,
} from "./lib.ts";

test("Diesel ORM Node Bindings - User loading and insertion", () => {
  const conn = establishConnection(":memory:");

  const inserted = users.insert(conn, [
    { name: "Sean", hairColor: "Black", email: "sean@example.com" },
    { name: "Gordon", hairColor: undefined, email: "gordon@example.com" },
  ]);

  assert.equal(inserted.length, 2);
  assert.equal(inserted[0]?.name, "Sean");
  assert.equal(inserted[0]?.hairColor, "Black");

  const loaded = users.load(conn);
  assert.equal(loaded.length, 2);
  assert.equal(loaded[1]?.name, "Gordon");
});

test("Diesel ORM Node Bindings - Belonging To queries", () => {
  const conn = establishConnection(":memory:");

  const [user1] = users.insert(conn, [{ name: "Alice", email: "alice@example.com" }]);

  assert.ok(user1);

  createPost(conn, "Alice Post 1", "Body 1", user1.id);
  createPost(conn, "Alice Post 2", "Body 2", user1.id);

  const posts = PostModel.belongingTo(user1).load(conn);
  assert.equal(posts.length, 2);
  assert.equal(posts[0]?.title, "Alice Post 1");
  assert.equal(posts[1]?.title, "Alice Post 2");
});

test("Diesel ORM Node Bindings - Post CRUD workflow", () => {
  const conn = establishConnection(":memory:");

  const post = createPost(conn, "Draft Title", "Draft Body");
  assert.equal(post.published, false);

  const fetchedDraft = getPost(conn, post.id);
  assert.ok(fetchedDraft);
  assert.equal(fetchedDraft.title, "Draft Title");

  const publishedListBefore = showPosts(conn);
  assert.equal(publishedListBefore.length, 0);

  const published = publishPost(conn, post.id);
  assert.equal(published.published, true);

  const publishedListAfter = showPosts(conn);
  assert.equal(publishedListAfter.length, 1);
  assert.equal(publishedListAfter[0]?.title, "Draft Title");

  const deletedCount = deletePost(conn, "Draft");
  assert.equal(deletedCount, 1);

  const publishedListFinal = showPosts(conn);
  assert.equal(publishedListFinal.length, 0);
});

test("Diesel ORM Node Bindings - Complex queries (Downloads)", () => {
  const conn = establishConnection(":memory:");

  const v1 = conn.insertVersion("1.0.0", 100);
  const v2 = conn.insertVersion("2.0.0", 100);

  conn.insertDownloads([
    { versionId: v1.id, downloads: 10, counted: 10, date: "2023-01-01" },
    { versionId: v2.id, downloads: 25, counted: 25, date: "2023-05-01" },
  ]);

  const downloads = conn.queryDownloads(100, "2023-02-01", 5);
  assert.equal(downloads.length, 1);
  assert.equal(downloads[0]?.downloads, 25);
});

test("Diesel ORM Node Bindings - Update by filter and raw SQL", () => {
  const conn = establishConnection(":memory:");

  users.insert(conn, [
    { name: "Spammer 1", email: "user1@spammer.com" },
    { name: "Spammer 2", email: "user2@spammer.com" },
    { name: "Legit User", email: "user@good.com" },
  ]);

  const bannedCount = conn.updateBannedByEmail("%@spammer.com", true);
  assert.equal(bannedCount, 2);

  const allUsers = users.load(conn);
  const bannedUsers = allUsers.filter((u) => u.banned);
  assert.equal(bannedUsers.length, 2);

  conn.execute("DELETE FROM users WHERE banned = 1");
  const remaining = users.load(conn);
  assert.equal(remaining.length, 1);
  assert.equal(remaining[0]?.name, "Legit User");
});
