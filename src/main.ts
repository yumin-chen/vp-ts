import console from "node:console";
import { Ksuid, KsuidMs, generateKsuid, parseKsuid } from "../index.js";

export const main = () => {
  const ksuid = Ksuid.now();
  const ksuidMs = KsuidMs.now();
  const generated = generateKsuid();
  const parsed = parseKsuid(ksuid.toBase62());

  return `Ksuid: ${ksuid.toBase62()}, KsuidMs: ${ksuidMs.toBase62()}, generated: ${generated}, equals: ${parsed.equals(ksuid)}`;
};

console.log(main());
