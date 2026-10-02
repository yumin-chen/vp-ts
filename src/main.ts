import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const native = require("../index.js");

export const Container = native.Container;
export const GetOrCreateResult = native.GetOrCreateResult;
export const ImageHandle = native.ImageHandle;
export const VolumeHandle = native.VolumeHandle;
export const ContainerCompose = native.ContainerCompose;
export const ContainerBuild = native.ContainerBuild;

export const main = () => {
  return "Container initialized";
};
