import * as native from "../../index.js";
import type { MachineActionResult, MachineCreateOptions, MachineListItem } from "../lib.ts";

export class ContainerMachineHandler {
  async create(options: MachineCreateOptions): Promise<MachineActionResult> {
    return native.createMachineCli(options);
  }

  async list(): Promise<MachineListItem[]> {
    return native.listMachinesCli();
  }
}

export const containerMachineHandler = new ContainerMachineHandler();
