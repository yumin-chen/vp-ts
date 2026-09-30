import console from "node:console";
import {
  Ksuid,
  type KsuidOptions,
  newKsuid,
  ksuidTimestampSeconds,
  crockfordEncode,
  crockfordDecode,
  ksuidToCrockford,
  ksuidFromCrockford,
} from "../index.js";

export { Ksuid, type KsuidOptions };

export const generateKsuid = () => {
  return newKsuid();
};

export const getKsuidTime = (id: string) => {
  return ksuidTimestampSeconds(id);
};

export const encodeCrockford = (n: number, customAlphabet?: string) => {
  return crockfordEncode(n, customAlphabet);
};

export const decodeCrockford = (input: string, customAlphabet?: string) => {
  return crockfordDecode(input, customAlphabet);
};

export const convertToCrockford = (id: string, customAlphabet?: string) => {
  return ksuidToCrockford(id, customAlphabet);
};

export const convertFromCrockford = (crockford: string, customAlphabet?: string) => {
  return ksuidFromCrockford(crockford, customAlphabet);
};

console.log("Generated KSUID:", generateKsuid());
