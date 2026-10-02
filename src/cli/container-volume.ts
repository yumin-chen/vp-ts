import * as native from "../../index.js";
import type { VolumeActionResult, VolumeCreateOptions, VolumeListItem } from "../lib.ts";

export class ContainerVolumeHandler {
  async create(options: VolumeCreateOptions): Promise<VolumeActionResult> {
    return native.createVolumeCli(options);
  }

  async list(): Promise<VolumeListItem[]> {
    return native.listVolumesCli();
  }
}

export const containerVolumeHandler = new ContainerVolumeHandler();
