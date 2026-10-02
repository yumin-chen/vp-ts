import { expect, test } from "vite-plus/test";
import { main, Container, ContainerCompose, ContainerBuild } from "./main.ts";

test("main returns Container initialized", () => {
  expect(main()).toBe("Container initialized");
});

test("Container, ContainerCompose, and ContainerBuild are defined", () => {
  expect(Container).toBeDefined();
  expect(ContainerCompose).toBeDefined();
  expect(ContainerBuild).toBeDefined();
});
