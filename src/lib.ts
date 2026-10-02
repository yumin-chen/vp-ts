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

export interface GetOrCreateResult {
  containerId: string;
  created: boolean;
  status: string;
}

export interface MountConfig {
  source: string;
  target: string;
  isVirtiofs: boolean;
}

export interface BuildOptions {
  contextDir?: string;
  file?: string;
  tag?: string[];
  buildArg?: string[];
  target?: string;
  cpus?: number;
  memory?: string;
  noCache?: boolean;
  output?: string[];
  platform?: string[];
  progress?: string;
  quiet?: boolean;
  secret?: string[];
  ssh?: string;
  pull?: boolean;
}

export interface BuildResult {
  success: boolean;
  imageId: string;
  tags: string[];
  message: string;
}

export interface BuilderStartOptions {
  cpus?: number;
  memory?: string;
  ssh?: boolean;
  dnsNameservers?: string[];
}

export interface BuilderStatusResult {
  running: boolean;
  containerId: string;
  cpus: number;
  memory: string;
}

export interface ContainerRunOptions {
  image: string;
  name?: string;
  detach?: boolean;
  interactive?: boolean;
  tty?: boolean;
  env?: string[];
  cpus?: number;
  memory?: string;
  mounts?: string[];
  ports?: string[];
  command?: string[];
}

export interface ContainerRunResult {
  containerId: string;
  status: string;
}

export interface ContainerStopOptions {
  containerIds: string[];
  all?: boolean;
  signal?: string;
  time?: number;
}

export interface ContainerListOptions {
  all?: boolean;
  format?: string;
  quiet?: boolean;
}

export interface ContainerListItem {
  id: string;
  image: string;
  status: string;
  name: string;
}

export interface ComposeUpOptions {
  file?: string;
  detach?: boolean;
  build?: boolean;
  services?: string[];
}

export interface ComposeDownOptions {
  file?: string;
  volumes?: boolean;
  rmi?: string;
}

export interface ComposeResult {
  success: boolean;
  services: string[];
  message: string;
}

export interface ImagePullOptions {
  reference: string;
  platform?: string;
  arch?: string;
  os?: string;
}

export interface ImageActionResult {
  success: boolean;
  reference: string;
  message: string;
}

export interface ImageListOptions {
  format?: string;
  quiet?: boolean;
  verbose?: boolean;
}

export interface ImageListItem {
  id: string;
  repository: string;
  tag: string;
  size: string;
}

export interface NetworkCreateOptions {
  name: string;
  internal?: boolean;
  subnet?: string;
}

export interface NetworkActionResult {
  success: boolean;
  name: string;
}

export interface NetworkListItem {
  name: string;
  plugin: string;
  internal: boolean;
}

export interface VolumeCreateOptions {
  name: string;
  size?: string;
}

export interface VolumeActionResult {
  success: boolean;
  name: string;
}

export interface VolumeListItem {
  name: string;
  driver: string;
}

export interface SystemStatusResult {
  running: boolean;
  apiVersion: string;
}

export interface MachineCreateOptions {
  image: string;
  name?: string;
  cpus?: number;
  memory?: string;
}

export interface MachineActionResult {
  success: boolean;
  name: string;
}

export interface MachineListItem {
  name: string;
  state: string;
  isDefault: boolean;
}

export interface K8sCreateOptions {
  name?: string;
  nodeImage?: string;
  cpus?: number;
  memory?: string;
}

export interface K8sActionResult {
  success: boolean;
  clusterName: string;
}

export const {
  BuildTransfer,
  Container,
  ImageTransfer,
  ServerStream,
  add,
  buildContainer,
  builderStart,
  builderStatus,
  builderStop,
  composeDown,
  composeUp,
  createMachineCli,
  createNetworkCli,
  createVolumeCli,
  getOrCreateContainer,
  k8SCreateCli,
  k8SDeleteCli,
  listContainersCli,
  listImagesCli,
  listMachinesCli,
  listNetworksCli,
  listVolumesCli,
  pullImageCli,
  runContainerCli,
  startContainer,
  stopContainerCli,
  systemStartCli,
  systemStatusCli,
  systemStopCli,
} = native;

export class VirtContainerClient {
  private containers = new Map<string, typeof native.Container.prototype>();

  async getOrCreate(id: string): Promise<GetOrCreateResult> {
    const res = native.getOrCreateContainer(id);
    if (!this.containers.has(id)) {
      const container = new native.Container(id, res.status);
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

export * from "./cli/container-build.ts";
export * from "./cli/container-compose.ts";
export * from "./cli/container-image.ts";
export * from "./cli/container-k8s.ts";
export * from "./cli/container-machine.ts";
export * from "./cli/container-network.ts";
export * from "./cli/container-system.ts";
export * from "./cli/container-volume.ts";
export * from "./cli/container.ts";
