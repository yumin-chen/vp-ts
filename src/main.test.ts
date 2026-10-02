import { expect, test } from "vite-plus/test";
import {
  BuildTransfer,
  Container,
  ImageTransfer,
  ServerStream,
  VirtContainerClient,
  containerBuildHandler,
  containerCliHandler,
  containerComposeHandler,
  getOrCreateContainer,
  main,
  startContainer,
} from "./main.ts";

test("main returns Hello, world!", () => {
  expect(main()).toBe("Hello, world!");
});

test("BuildTransfer metadata extensions work as expected", () => {
  const bt = new BuildTransfer({
    stage: "builder",
    method: "dockerfile",
    "include-patterns": "src/*,package.json",
    followpaths: "node_modules",
    mode: "copy",
    size: "1024",
    offset: "0",
    length: "512",
  });

  expect(bt.stage()).toBe("builder");
  expect(bt.method()).toBe("dockerfile");
  expect(bt.includePatterns()).toEqual(["src/*", "package.json"]);
  expect(bt.followPaths()).toEqual(["node_modules"]);
  expect(bt.mode()).toBe("copy");
  expect(bt.size()).toBe(1024);
  expect(bt.offset()).toBe(0);
  expect(bt.len()).toBe(512);
});

test("ImageTransfer metadata extensions work as expected", () => {
  const img = new ImageTransfer({
    stage: "final",
    method: "pull",
    ref: "alpine:latest",
    platform: "linux/amd64",
    mode: "layer",
    size: "2048",
    length: "1024",
    offset: "100",
  });

  expect(img.stage()).toBe("final");
  expect(img.method()).toBe("pull");
  expect(img.refName()).toBe("alpine:latest");
  expect(img.platform()).toBe("linux/amd64");
  expect(img.mode()).toBe("layer");
  expect(img.size()).toBe(2048);
  expect(img.len()).toBe(1024);
  expect(img.offset()).toBe(100);
});

test("ServerStream wraps transfers correctly", () => {
  const img = new ImageTransfer({ ref: "ubuntu:22.04" });
  const bt = new BuildTransfer({ stage: "compile" });
  const stream = new ServerStream(img, bt, {
    stdin: "input",
    stdout: "output",
    tty: true,
  });

  expect(stream.getImageTransfer()?.refName()).toBe("ubuntu:22.04");
  expect(stream.getBuildTransfer()?.stage()).toBe("compile");
  expect(stream.getIo()).toEqual({
    stdin: "input",
    stdout: "output",
    tty: true,
  });
});

test("Container (refactored without Js prefix) and getOrCreateContainer API", () => {
  const c = new Container("test-container", "stopped");
  expect(c.getId()).toBe("test-container");
  expect(c.getStatus()).toBe("stopped");

  c.setMetadata("env", "production");
  expect(c.getMetadata("env")).toBe("production");

  const created = getOrCreateContainer("new-container");
  expect(created.containerId).toBe("new-container");
  expect(created.created).toBe(true);
});

test("startContainer behavior and VirtContainerClient", async () => {
  const res = startContainer("cont-1", false, false);
  expect(res.success).toBe(true);
  expect(res.detached).toBe(true);

  const client = new VirtContainerClient();
  const created = await client.getOrCreate("c123");
  expect(created.containerId).toBe("c123");

  const startRes = await client.start("c123", { attach: false });
  expect(startRes.success).toBe(true);

  const list = await client.list();
  expect(list.length).toBeGreaterThan(0);
});

test("CLI subhandlers (container, build, compose)", async () => {
  const buildRes = await containerBuildHandler.build({
    contextDir: ".",
    tag: ["my-app:latest"],
  });
  expect(buildRes.success).toBe(true);
  expect(buildRes.tags).toEqual(["my-app:latest"]);

  const builderStatus = await containerBuildHandler.statusBuilder();
  expect(builderStatus.running).toBe(true);

  const runRes = await containerCliHandler.run({
    image: "ubuntu:latest",
    name: "test-run",
    detach: true,
  });
  expect(runRes.containerId).toBe("test-run");
  expect(runRes.status).toBe("running");

  const listRes = await containerCliHandler.list();
  expect(listRes.length).toBeGreaterThan(0);

  const composeRes = await containerComposeHandler.up({
    services: ["web", "db"],
  });
  expect(composeRes.success).toBe(true);
  expect(composeRes.services).toEqual(["web", "db"]);
});
