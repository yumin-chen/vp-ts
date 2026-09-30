import console from "node:console";
import { parse, stringify } from "../index.js";

export const main = () => {
  const parsed = parse('key = "value"\n');
  const stringified = stringify(parsed);
  return `Parsed key: ${parsed.key}, Stringified: ${stringified.trim()}`;
};

console.log(main());
