import * as native from "../../index.js";
import type { NetworkActionResult, NetworkCreateOptions, NetworkListItem } from "../lib.ts";

export class ContainerNetworkHandler {
  async create(options: NetworkCreateOptions): Promise<NetworkActionResult> {
    return native.createNetworkCli(options);
  }

  async list(): Promise<NetworkListItem[]> {
    return native.listNetworksCli();
  }
}

export const containerNetworkHandler = new ContainerNetworkHandler();
