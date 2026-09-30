import { decodeCrockford, encodeCrockford, Ksuid, KsuidMs, type KsuidOptions } from "../index.js";

export { decodeCrockford, encodeCrockford, Ksuid, KsuidMs, type KsuidOptions };

export const main = () => {
  const ksuid = Ksuid.now();
  return `KSUID: ${ksuid.toString()}`;
};
