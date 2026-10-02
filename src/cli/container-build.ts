import { createRequire } from "node:module";
import type { BuildOptions, BuilderStartOptions, BuilderStatus } from "../../index.d.ts";

const require = createRequire(import.meta.url);
const native = require("../../index.js");

const builderInstance = new native.ContainerBuild();

export const Container = {
  build: (options?: BuildOptions): string => builderInstance.build(options),
  builderStart: (options?: BuilderStartOptions): boolean => builderInstance.builderStart(options),
  builderStatus: (): BuilderStatus => builderInstance.builderStatus(),
  builderStop: (): boolean => builderInstance.builderStop(),
  builderDelete: (): boolean => builderInstance.builderDelete(),
};
