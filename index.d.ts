export class Container {
  constructor(options?: Options);
  static withDefaultConfig(): Container;
  static initDefault(options?: Options): void;
  static rest(options: ContainerRestOptions): Container;
  importContainer(archivePath: string, name?: string): Container;
  create(options?: ContainerOptions, name?: string): Container;
  getOrCreate(options?: ContainerOptions, name?: string): GetOrCreateResult;
  listInfo(): Array<ContainerInfo>;
  getInfo(idOrName: string): ContainerInfo | null;
  get(idOrName: string): Container | null;
  metrics(): RuntimeMetrics;
  get images(): ImageHandle;
  get volumes(): VolumeHandle;
  remove(idOrName: string, force?: boolean): void;
  close(): void;
  shutdown(timeout?: number): void;
  exec(command: string, args?: Array<string>): string;
}

export class GetOrCreateResult {
  get created(): boolean;
  get container(): Container;
}

export class ImageHandle {
  list(): Array<string>;
}

export class VolumeHandle {
  list(): Array<string>;
}

export class ContainerCompose {
  constructor();
  status(options?: ComposeStatusOptions): boolean;
  generateKey(options?: GenerateKeyOptions): string;
  generateCert(options?: GenerateCertOptions): boolean;
  listKeys(options?: ListKeysOptions): Array<string>;
  revokeKey(options: RevokeKeyOptions): boolean;
}

export class ContainerBuild {
  constructor();
  build(options?: BuildOptions): string;
  builderStart(options?: BuilderStartOptions): boolean;
  builderStatus(): BuilderStatus;
  builderStop(): boolean;
  builderDelete(): boolean;
}

export interface Options {
  homeDir?: string;
}

export interface ContainerOptions {
  image?: string;
  memoryMib?: number;
  cpus?: number;
}

export interface ContainerRestOptions {
  endpoint?: string;
}

export interface ContainerInfo {
  id: string;
  name?: string;
  status: string;
}

export interface RuntimeMetrics {
  containersCreatedTotal: number;
  numRunningContainers: number;
}

export interface ComposeStatusOptions {
  socket?: string;
  address?: string;
  cacert?: string;
}

export interface GenerateKeyOptions {
  name?: string;
  authFile?: string;
}

export interface GenerateCertOptions {
  outDir?: string;
  cn?: string;
  days?: number;
  sanDns?: Array<string>;
  sanIp?: Array<string>;
  force?: boolean;
}

export interface ListKeysOptions {
  authFile?: string;
}

export interface RevokeKeyOptions {
  name: string;
  authFile?: string;
}

export interface BuildOptions {
  contextDir?: string;
  file?: string;
  target?: string;
  tag?: Array<string>;
  cpus?: number;
  memory?: string;
  noCache?: boolean;
  pull?: boolean;
  secret?: Array<string>;
  ssh?: string;
  vsockPort?: number;
}

export interface BuilderStartOptions {
  cpus?: number;
  memory?: string;
  ssh?: string;
  dns?: Array<string>;
}

export interface BuilderStatus {
  running: boolean;
  cpus: number;
  memory: string;
}
