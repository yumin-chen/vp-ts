import console from "node:console";
import { VirtContainerClient, containerClient } from "./lib.ts";

export * from "./lib.ts";

export const main = () => {
  return "Hello, world!";
};

if (process.env.NODE_ENV !== "test") {
  console.log(main());
}
