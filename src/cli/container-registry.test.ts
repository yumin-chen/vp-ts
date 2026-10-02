import { expect, test } from "vite-plus/test";
import { Container } from "./container-registry.ts";

test("Container.registryLogin returns boolean", () => {
  const result = Container.registryLogin({ server: "ghcr.io", username: "user" });
  expect(result).toBe(true);
});

test("Container.registryLogout returns boolean", () => {
  const result = Container.registryLogout({ registry: "ghcr.io" });
  expect(result).toBe(true);
});

test("Container.registryList returns empty array by default", () => {
  const list = Container.registryList();
  expect(Array.isArray(list)).toBe(true);
  expect(list.length).toBe(0);
});
