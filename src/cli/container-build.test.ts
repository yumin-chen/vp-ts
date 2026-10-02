import { expect, test } from "vite-plus/test";
import { Container } from "./container-build.ts";

test("Container.build returns build message", () => {
  const result = Container.build({ tag: ["my-app:v1"] });
  expect(result).toBe("Successfully built my-app:v1");
});

test("Container.builderStatus returns status object", () => {
  const status = Container.builderStatus();
  expect(status.running).toBe(true);
  expect(status.cpus).toBe(2);
  expect(status.memory).toBe("2048MB");
});

test("Container.builderStart, builderStop, and builderDelete return boolean", () => {
  expect(Container.builderStart({ cpus: 4 })).toBe(true);
  expect(Container.builderStop()).toBe(true);
  expect(Container.builderDelete()).toBe(true);
});
