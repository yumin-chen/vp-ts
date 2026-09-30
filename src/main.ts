import console from "node:console";
import { newKsuid, ksuidTimestampSeconds } from "../index.js";

export const generateKsuid = () => {
  return newKsuid();
};

export const getKsuidTime = (id: string) => {
  return ksuidTimestampSeconds(id);
};

console.log("Generated KSUID:", generateKsuid());
