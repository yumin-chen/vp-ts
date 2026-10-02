import { expect, test } from "vite-plus/test";
import { ContainerRegistry, containerRegistryHandler } from "./container-registry.ts";

test("ContainerRegistry.login and logout", async () => {
  const loginRes = await ContainerRegistry.login({
    server: "docker.io",
    username: "testuser",
    password: "secretpassword",
  });

  expect(loginRes.success).toBe(true);
  expect(loginRes.server).toBe("docker.io");

  const logoutRes = await ContainerRegistry.logout("docker.io");
  expect(logoutRes.success).toBe(true);
});

test("ContainerRegistry.list and loadConfig", async () => {
  const listRes = await ContainerRegistry.list();
  expect(listRes.length).toBeGreaterThan(0);
  expect(listRes[0].hostname).toBe("docker.io");

  const config = await ContainerRegistry.loadConfig();
  expect(config.build?.cpus).toBe(2);
  expect(config.container?.cpus).toBe(4);
});

test("containerRegistryHandler wrapper class calls ContainerRegistry functions", async () => {
  const loginRes = await containerRegistryHandler.login({ server: "ghcr.io" });
  expect(loginRes.success).toBe(true);
  expect(loginRes.server).toBe("ghcr.io");
});
