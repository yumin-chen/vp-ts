import * as native from "../../index.js";
import type { SystemStatusResult } from "../lib.ts";

export class ContainerSystemHandler {
  async status(): Promise<SystemStatusResult> {
    return native.systemStatusCli();
  }

  async start(): Promise<boolean> {
    return native.systemStartCli();
  }

  async stop(): Promise<boolean> {
    return native.systemStopCli();
  }
}

export const containerSystemHandler = new ContainerSystemHandler();
