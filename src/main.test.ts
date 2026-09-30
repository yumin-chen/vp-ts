import { expect, test } from "vite-plus/test";
import { establishConnection, DieselOrm } from "./main.ts";

test("establishConnection creates a working DieselOrm instance", () => {
  const db = establishConnection(":memory:");
  expect(db).toBeInstanceOf(DieselOrm);
});

test("user operations: insertUsers and loadAllUsers", () => {
  const db = establishConnection(":memory:");

  // Initially empty
  const initialUsers = db.loadAllUsers();
  expect(initialUsers).toEqual([]);

  // Insert users
  const newUsers = [
    { name: "Sean", hairColor: "Black" },
    { name: "Gordon", hairColor: undefined },
  ];

  const insertedUsers = db.insertUsers(newUsers);
  expect(insertedUsers.length).toBe(2);
  expect(insertedUsers[0].name).toBe("Sean");
  expect(insertedUsers[0].hairColor).toBe("Black");
  expect(insertedUsers[0].banned).toBe(false);

  expect(insertedUsers[1].name).toBe("Gordon");
  expect(insertedUsers[1].hairColor).toBeUndefined();

  // Load all users
  const allUsers = db.loadAllUsers();
  expect(allUsers.length).toBe(2);
  expect(allUsers[0].id).toBe(insertedUsers[0].id);
  expect(allUsers[1].id).toBe(insertedUsers[1].id);
});

test("post operations: createPost, loadPostsForUser, publishPost, showPublishedPosts, getPost, deletePosts", () => {
  const db = establishConnection(":memory:");

  // Create user
  const [user] = db.insertUsers([{ name: "Alice", hairColor: "Blonde" }]);

  // Create posts for user
  const post1 = db.createPost({
    userId: user.id,
    title: "First Diesel Post",
    body: "This is the body of the first post.",
    published: false,
  });

  const post2 = db.createPost({
    userId: user.id,
    title: "Second Diesel Post",
    body: "This is the body of the second post.",
    published: false,
  });

  expect(post1.id).toBeDefined();
  expect(post1.userId).toBe(user.id);
  expect(post1.published).toBe(false);

  // Load posts for user
  const userPosts = db.loadPostsForUser(user.id);
  expect(userPosts.length).toBe(2);
  expect(userPosts.map((p) => p.title)).toContain("First Diesel Post");

  // Get single post
  const fetchedPost = db.getPost(post1.id);
  expect(fetchedPost).not.toBeNull();
  expect(fetchedPost?.title).toBe("First Diesel Post");

  const nonExistentPost = db.getPost(9999);
  expect(nonExistentPost).toBeNull();

  // Initially no published posts
  expect(db.showPublishedPosts()).toEqual([]);

  // Publish post1
  const publishedPost1 = db.publishPost(post1.id);
  expect(publishedPost1.published).toBe(true);

  // Show published posts
  const publishedList = db.showPublishedPosts(5);
  expect(publishedList.length).toBe(1);
  expect(publishedList[0].title).toBe("First Diesel Post");

  // Delete post matching title pattern
  const deletedCount = db.deletePosts("Second%");
  expect(deletedCount).toBe(1);

  const remainingUserPosts = db.loadPostsForUser(user.id);
  expect(remainingUserPosts.length).toBe(1);
  expect(remainingUserPosts[0].title).toBe("First Diesel Post");
});

test("batch update: batchBanUsers", () => {
  const db = establishConnection(":memory:");

  db.insertUsers([
    { name: "spammer1", hairColor: "None" },
    { name: "spammer2", hairColor: "None" },
    { name: "legitUser", hairColor: "Brown" },
  ]);

  const bannedCount = db.batchBanUsers("spammer%");
  expect(bannedCount).toBe(2);

  const users = db.loadAllUsers();
  const bannedUsers = users.filter((u) => u.banned);
  expect(bannedUsers.length).toBe(2);
  expect(bannedUsers.map((u) => u.name)).toEqual(["spammer1", "spammer2"]);
});

test("complex queries: downloads and recent downloads filtering", () => {
  const db = establishConnection(":memory:");

  db.insertDownloads([
    { versionId: 1, downloads: 100, counted: 1, date: "2023-01-01" },
    { versionId: 1, downloads: 200, counted: 1, date: "2023-05-01" },
    { versionId: 2, downloads: 150, counted: 1, date: "2023-06-01" },
    { versionId: 3, downloads: 300, counted: 1, date: "2023-07-01" },
  ]);

  const recentDownloads = db.getRecentDownloadsForVersions([1, 2], "2023-02-01", 5);
  expect(recentDownloads.length).toBe(2);
  expect(recentDownloads[0].versionId).toBe(1);
  expect(recentDownloads[0].date).toBe("2023-05-01");
  expect(recentDownloads[1].versionId).toBe(2);
  expect(recentDownloads[1].date).toBe("2023-06-01");
});

test("raw SQL execution", () => {
  const db = establishConnection(":memory:");

  const affected = db.executeRawSql(
    "INSERT INTO users (name, hair_color) VALUES ('RawUser', 'Red')",
  );
  expect(affected).toBe(1);

  const users = db.loadAllUsers();
  expect(users.length).toBe(1);
  expect(users[0].name).toBe("RawUser");
});
