import * as native from "../../index.js";
import type {
  ContainerListItem,
  ContainerListOptions,
  ContainerRunOptions,
  ContainerRunResult,
  ContainerStopOptions,
} from "../lib.ts";

export class ContainerCliHandler {
  async run(options: ContainerRunOptions): Promise<ContainerRunResult> {
    return native.runContainerCli(options);
  }

  async stop(options: ContainerStopOptions): Promise<string[]> {
    return native.stopContainerCli(options);
  }

  async list(options?: ContainerListOptions): Promise<ContainerListItem[]> {
    return native.listContainersCli(options);
  }
}

export const containerCliHandler = new ContainerCliHandler();
