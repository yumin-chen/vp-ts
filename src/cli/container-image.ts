import * as native from "../../index.js";
import type { ImageActionResult, ImageListItem, ImageListOptions, ImagePullOptions } from "../lib.ts";

export class ContainerImageHandler {
  async pull(options: ImagePullOptions): Promise<ImageActionResult> {
    return native.pullImageCli(options);
  }

  async list(options?: ImageListOptions): Promise<ImageListItem[]> {
    return native.listImagesCli(options);
  }
}

export const containerImageHandler = new ContainerImageHandler();
