import console from "node:console";
import { Ksuid, KsuidMs, generateKsuid, parseKsuid } from "../index.js";

export const main = () => {
  const ksuid32 = new Ksuid({ enc: "base32", timestampSize: "32bit" });
  const ksuid48 = new Ksuid({ enc: "base32", timestampSize: "48bit" });
  const ksuidMs = KsuidMs.now();
  const generated = generateKsuid();
  const parsed = parseKsuid(ksuid32.toBase62());

  return `Ksuid (base32 32bit): ${ksuid32.toString()}, Ksuid (base32 48bit): ${ksuid48.toString()}, KsuidMs: ${ksuidMs.toBase62()}, generated: ${generated}, equals: ${parsed.equals(ksuid32)}`;
};

console.log(main());
