import console from "node:console";
// @ts-ignore
import { add } from "../index.js";

export const main = () => {
  return "Hello, world!";
};

export { add };

console.log(main());
console.log("2 + 3 =", add(2, 3));
