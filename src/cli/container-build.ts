import * as native from "../../index.js";
import type { BuildOptions, BuildResult, BuilderStartOptions, BuilderStatusResult } from "../lib.ts";

export namespace Container {
  export async function build(options?: BuildOptions): Promise<BuildResult> {
    return native.buildContainer(options);
  }

  export async function startBuilder(options?: BuilderStartOptions): Promise<BuilderStatusResult> {
    return native.builderStart(options);
  }

  export async function statusBuilder(): Promise<BuilderStatusResult> {
    return native.builderStatus();
  }

  export async function stopBuilder(): Promise<boolean> {
    return native.builderStop();
  }
}

export class ContainerBuildHandler {
  async build(options?: BuildOptions): Promise<BuildResult> {
    return Container.build(options);
  }

  async startBuilder(options?: BuilderStartOptions): Promise<BuilderStatusResult> {
    return Container.startBuilder(options);
  }

  async statusBuilder(): Promise<BuilderStatusResult> {
    return Container.statusBuilder();
  }

  async stopBuilder(): Promise<boolean> {
    return Container.stopBuilder();
  }
}

export const containerBuildHandler = new ContainerBuildHandler();
