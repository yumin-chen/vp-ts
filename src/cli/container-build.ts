import * as native from "../../index.js";
import type { BuildOptions, BuildResult, BuilderStartOptions, BuilderStatusResult } from "../lib.ts";

export class ContainerBuildHandler {
  async build(options?: BuildOptions): Promise<BuildResult> {
    return native.buildContainer(options);
  }

  async startBuilder(options?: BuilderStartOptions): Promise<BuilderStatusResult> {
    return native.builderStart(options);
  }

  async statusBuilder(): Promise<BuilderStatusResult> {
    return native.builderStatus();
  }

  async stopBuilder(): Promise<boolean> {
    return native.builderStop();
  }
}

export const containerBuildHandler = new ContainerBuildHandler();
