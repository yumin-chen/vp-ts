import * as native from "../../index.js";
import type { K8sActionResult, K8sCreateOptions } from "../lib.ts";

export class ContainerK8sHandler {
  async create(options?: K8sCreateOptions): Promise<K8sActionResult> {
    return (native as any).k8SCreateCli(options);
  }

  async delete(name?: string): Promise<boolean> {
    return (native as any).k8SDeleteCli(name);
  }
}

export const containerK8sHandler = new ContainerK8sHandler();
