import { expect, test } from "vite-plus/test";
import { Container, containerBuildHandler } from "./container-build.ts";

test("Container.build runs programmable napi build bindings", async () => {
  const result = await Container.build({
    contextDir: ".",
    file: "Dockerfile",
    tag: ["my-app:v1.0.0"],
  });

  expect(result.success).toBe(true);
  expect(result.tags).toEqual(["my-app:v1.0.0"]);
  expect(result.message).toBe("Build completed successfully");
});

test("Container builder commands (startBuilder, statusBuilder, stopBuilder)", async () => {
  const startRes = await Container.startBuilder({ cpus: 4, memory: "4096MB" });
  expect(startRes.running).toBe(true);
  expect(startRes.cpus).toBe(4);
  expect(startRes.memory).toBe("4096MB");

  const statusRes = await Container.statusBuilder();
  expect(statusRes.running).toBe(true);
  expect(statusRes.containerId).toBe("buildkit");

  const stopRes = await Container.stopBuilder();
  expect(stopRes).toBe(true);
});

test("containerBuildHandler instance wrapper calls Container functions", async () => {
  const buildRes = await containerBuildHandler.build({ tag: ["test-tag"] });
  expect(buildRes.success).toBe(true);
  expect(buildRes.tags).toEqual(["test-tag"]);
});
