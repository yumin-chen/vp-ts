import * as native from "../index.js";

export interface ClientStream {
  streamId: string;
  metadata: Record<string, string>;
}

export interface ContainerConfig {
  id: string;
  status: string;
  mounts: Array<MountConfig>;
  terminal: boolean;
}

export interface ContainerStartOptions {
  attach: boolean;
  interactive: boolean;
  containerId: string;
}

export interface ContainerStartResult {
  success: boolean;
  exitCode: number;
  containerId: string;
  detached: boolean;
}

export interface InfoRequest {
  id: string;
  query?: string;
}

export interface InfoResponse {
  id: string;
  status: string;
  info: Record<string, string>;
}

export interface Io {
  stdin?: string;
  stdout?: string;
  stderr?: string;
  tty: boolean;
}

export interface JsGetOrCreateResult {
  containerId: string;
  created: boolean;
  status: string;
}

export interface MountConfig {
  source: string;
  target: string;
  isVirtiofs: boolean;
}

export const {
  BuildTransfer,
  ImageTransfer,
  JsContainer,
  ServerStream,
  add,
  getOrCreateContainer,
  startContainer,
} = native;

export class VirtContainerClient {
  private containers = new Map<string, typeof native.JsContainer.prototype>();

  async getOrCreate(id: string): Promise<JsGetOrCreateResult> {
    const res = native.getOrCreateContainer(id);
    if (!this.containers.has(id)) {
      const container = new native.JsContainer(id, res.status);
      this.containers.set(id, container);
    }
    return res;
  }

  async start(
    id: string,
    options?: { attach?: boolean; interactive?: boolean; configJson?: string }
  ): Promise<ContainerStartResult> {
    return native.startContainer(
      id,
      options?.attach ?? false,
      options?.interactive ?? false,
      options?.configJson
    );
  }

  async stop(id: string): Promise<{ success: boolean; containerId: string }> {
    return {
      success: true,
      containerId: id,
    };
  }

  async list(): Promise<Array<{ id: string; status: string }>> {
    const result: Array<{ id: string; status: string }> = [];
    for (const [id, container] of this.containers.entries()) {
      result.push({
        id,
        status: container.getStatus(),
      });
    }
    return result;
  }
}

export const containerClient = new VirtContainerClient();
