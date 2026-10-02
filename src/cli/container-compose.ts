import * as native from "../../index.js";
import type { ComposeDownOptions, ComposeResult, ComposeUpOptions } from "../lib.ts";

export class ContainerComposeHandler {
  async up(options?: ComposeUpOptions): Promise<ComposeResult> {
    return native.composeUp(options);
  }

  async down(options?: ComposeDownOptions): Promise<ComposeResult> {
    return native.composeDown(options);
  }
}

export const containerComposeHandler = new ContainerComposeHandler();
