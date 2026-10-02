import { createRequire } from "node:module";
import type {
  RegistryListOptions,
  RegistryLoginOptions,
  RegistryLogoutOptions,
  RegistryResource,
} from "../../index.d.ts";

const require = createRequire(import.meta.url);
const native = require("../../index.js");

const registryInstance = new native.ContainerRegistry();

export const Container = {
  registryLogin: (options: RegistryLoginOptions): boolean => registryInstance.login(options),
  registryLogout: (options: RegistryLogoutOptions): boolean => registryInstance.logout(options),
  registryList: (options?: RegistryListOptions): Array<RegistryResource> =>
    registryInstance.list(options),
};
