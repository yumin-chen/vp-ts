import { Ksuid, KsuidMs } from "../index.js";

export { Ksuid, KsuidMs };

export const main = () => {
  const ksuid = Ksuid.now();
  return `KSUID: ${ksuid.toString()}`;
};
