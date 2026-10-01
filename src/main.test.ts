import { expect, test } from "vite-plus/test";
import {
  JsContainer,
  JsContainerRestOptions,
  UVPythonRuntime,
  VirtcontainerRuntime,
} from "./main.ts";

test("VirtcontainerRuntime default initialization and container creation", async () => {
  const runtime = VirtcontainerRuntime.withDefaultConfig();
  const handle = await runtime.create({ image: "alpine:latest" }, "test-alpine");
  expect(handle).toBeDefined();

  const execRes = await handle.exec("echo", ["hello"]);
  expect(execRes).toBe("Executed: echo hello");

  const info = await runtime.getInfo("test-alpine");
  expect(info?.name).toBe("test-alpine");
  expect(info?.image).toBe("alpine:latest");

  const list = await runtime.listInfo();
  expect(list.length).toBeGreaterThan(0);

  const metrics = await runtime.metrics();
  expect(metrics.containersCreatedTotal).toBeGreaterThan(0);
});

test("UVPythonRuntime generates command args with fusevm config", async () => {
  const uv = new UVPythonRuntime({
    pythonVersion: "3.11",
    packages: ["numpy", "pandas"],
    fusevm: { version: "0.26.8", enableJit: true },
  });

  const args = uv.getCommandArgs();
  expect(args).toEqual(["uv", "run", "--python", "3.11", "--with", "numpy", "pandas"]);
});

test("JsContainer NAPI direct methods", async () => {
  const container = new JsContainer({ homeDir: "/tmp/test-container" });
  const handle = await container.create({ image: "python:slim" }, "py-ctr");
  expect(handle).toBeDefined();

  const getResult = await container.getOrCreate({ image: "python:slim" }, "py-ctr");
  expect(getResult.created).toBe(false);
  expect(getResult.container).toBeDefined();

  const restOpts = new JsContainerRestOptions("http://localhost:8080", "token123");
  const restContainer = JsContainer.rest(restOpts);
  expect(restContainer).toBeDefined();

  await container.remove("py-ctr", true);
  const info = await container.getInfo("py-ctr");
  expect(info).toBeNull();
});
