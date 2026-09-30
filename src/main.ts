import console from "node:console";
import Sqids from "../index.js";

export const main = () => {
  const sqids = new Sqids();
  const id = sqids.encode([1, 2, 3]);
  const numbers = sqids.decode(id);
  return `Encoded: ${id}, Decoded: ${numbers.join(",")}`;
};

console.log(main());
