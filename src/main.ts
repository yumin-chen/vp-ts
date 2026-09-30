import console from "node:console";
import { add } from "../index.js";

export const main = () => {
  return `Hello, world! 2 + 3 = ${add(2, 3)}`;
};

console.log(main());
