import * as native from "../../index.js";
import type { K8sActionResult, K8sCreateOptions } from "../lib.ts";

export class ContainerK8sHandler {
  async create(options?: K8sCreateOptions): Promise<K8sActionResult> {
    return native.k8sCreateCli(options);
  }

  async delete(name?: string): Promise<boolean> {
    return native.k8sDeleteCli(name);
  }
}

export const containerK8sHandler = new ContainerK8sHandler();
