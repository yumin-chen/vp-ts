import * as native from "../../index.js";
import type {
  ContainerSystemConfig,
  RegistryActionResult,
  RegistryListItem,
  RegistryLoginOptions,
} from "../lib.ts";

export namespace ContainerRegistry {
  export async function login(options: RegistryLoginOptions): Promise<RegistryActionResult> {
    return native.registryLogin(options);
  }

  export async function logout(server: string): Promise<RegistryActionResult> {
    return native.registryLogout(server);
  }

  export async function list(): Promise<RegistryListItem[]> {
    return native.registryList();
  }

  export async function loadConfig(configPath?: string): Promise<ContainerSystemConfig> {
    return native.loadContainerSystemConfig(configPath);
  }
}

export class ContainerRegistryHandler {
  async login(options: RegistryLoginOptions): Promise<RegistryActionResult> {
    return ContainerRegistry.login(options);
  }

  async logout(server: string): Promise<RegistryActionResult> {
    return ContainerRegistry.logout(server);
  }

  async list(): Promise<RegistryListItem[]> {
    return ContainerRegistry.list();
  }

  async loadConfig(configPath?: string): Promise<ContainerSystemConfig> {
    return ContainerRegistry.loadConfig(configPath);
  }
}

export const containerRegistryHandler = new ContainerRegistryHandler();
