import console from "node:console";
import { Ksuid } from "../index.js";

export const main = () => {
  const ksuid = Ksuid.now();
  return `Ksuid generated: ${ksuid.toString()}`;
};

console.log(main());
