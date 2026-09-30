import console from "node:console";
import { createPost, establishConnection, PostModel, publishPost, users } from "./lib.ts";

export * from "./lib.ts";

export const main = () => {
  const conn = establishConnection(":memory:");

  // Insert users
  const insertedUsers = users.insert(conn, [
    { name: "Sean", hairColor: "Black", email: "sean@example.com" },
    { name: "Gordon", email: "gordon@example.com" },
  ]);

  const allUsers = users.load(conn);

  const firstUser = allUsers[0] ?? { id: 1 };
  const newPost = createPost(
    conn,
    "Diesel ORM Node Bindings",
    "Building fast NAPI bindings",
    firstUser.id,
  );
  publishPost(conn, newPost.id);

  const userPosts = PostModel.belongingTo(firstUser).load(conn);

  return {
    usersCount: allUsers.length,
    insertedCount: insertedUsers.length,
    userPostsCount: userPosts.length,
  };
};

if (process.env["NODE_ENV"] !== "test") {
  console.log(main());
}
