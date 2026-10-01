import * as nativeBinding from "../native/index.js";

export const JsContainer = nativeBinding.JsContainer;
export const JsContainerHandle = nativeBinding.JsContainerHandle;
export const JsContainerRestOptions = nativeBinding.JsContainerRestOptions;
export const JsGetOrCreateResult = nativeBinding.JsGetOrCreateResult;
export const JsImageHandle = nativeBinding.JsImageHandle;
export const JsVolumeHandle = nativeBinding.JsVolumeHandle;

export interface JsContainerState {
  status: string;
}

export interface JsContainerInfo {
  id: string;
  name?: string;
  state: JsContainerState;
  image: string;
}

export interface JsContainerOptions {
  image: string;
  memoryMib?: number;
  cpus?: number;
  volumes?: string[];
  env?: string[];
  cmd?: string[];
}

export interface JsOptions {
  homeDir?: string;
}

export interface JsRuntimeMetrics {
  containersCreatedTotal: number;
  numRunningContainers: number;
}

/**
 * FuseVM runtime configuration helper for virtcontainers.
 */
export interface FuseVMConfig {
  version: string;
  enableJit?: boolean;
  cacheDir?: string;
}

export const DEFAULT_FUSEVM_CONFIG: FuseVMConfig = {
  version: "0.26.8",
  enableJit: true,
};

/**
 * Python runtime helper using uv and fusevm.
 */
export interface UVPythonRuntimeOptions {
  pythonVersion?: string;
  packages?: string[];
  fusevm?: FuseVMConfig;
}

export class UVPythonRuntime {
  private options: UVPythonRuntimeOptions;

  constructor(options: UVPythonRuntimeOptions = {}) {
    this.options = {
      pythonVersion: options.pythonVersion ?? "3.12",
      packages: options.packages ?? [],
      fusevm: options.fusevm ?? DEFAULT_FUSEVM_CONFIG,
    };
  }

  public getCommandArgs(): string[] {
    const pkgs = this.options.packages?.length ? ["--with", ...this.options.packages] : [];
    return ["uv", "run", "--python", this.options.pythonVersion!, ...pkgs];
  }
}

/**
 * Virtcontainers (runtime-rs) Node binding wrapper around NAPI JsContainer.
 */
export class VirtcontainerRuntime {
  private container: any;

  constructor(options?: JsOptions) {
    this.container = options ? new JsContainer(options) : JsContainer.withDefaultConfig();
  }

  public static withDefaultConfig(): VirtcontainerRuntime {
    return new VirtcontainerRuntime();
  }

  public static initDefault(options: JsOptions): void {
    JsContainer.initDefault(options);
  }

  public static rest(options: any): VirtcontainerRuntime {
    const instance = new VirtcontainerRuntime();
    instance.container = JsContainer.rest(options);
    return instance;
  }

  public async create(options: JsContainerOptions, name?: string) {
    return this.container.create(options, name);
  }

  public async createPythonRuntime(uvOpts: UVPythonRuntimeOptions, name?: string) {
    const uv = new UVPythonRuntime(uvOpts);
    const cmd = uv.getCommandArgs();
    return this.container.create(
      {
        image: "python:slim",
        cmd,
      },
      name,
    );
  }

  public async getOrCreate(options: JsContainerOptions, name?: string) {
    return this.container.getOrCreate(options, name);
  }

  public async listInfo() {
    return this.container.listInfo();
  }

  public async getInfo(idOrName: string) {
    return this.container.getInfo(idOrName);
  }

  public async get(idOrName: string) {
    return this.container.get(idOrName);
  }

  public async metrics() {
    return this.container.metrics();
  }

  public get images() {
    return this.container.images;
  }

  public get volumes() {
    return this.container.volumes;
  }

  public async remove(idOrName: string, force?: boolean) {
    return this.container.remove(idOrName, force);
  }

  public async shutdown(timeout?: number) {
    return this.container.shutdown(timeout);
  }
}
