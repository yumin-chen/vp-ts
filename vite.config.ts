import { defineConfig } from "vite-plus";

export default defineConfig({
  pack: {
    entry: ["./src/main.ts"],
    format: "esm",
    outDir: "dist",
    exports: true,
    dts: {
      generator: "tsgo",
    },
    deps: {
      resolveDepSubpath: true,
    },
  },
  test: {
    include: ["src/**/*.{test,spec}.ts"],
  },
  staged: {
    "*": "vp check --fix",
  },
  fmt: {
    ignore: ["examples/**"],
  },
  lint: {
    ignorePatterns: ["examples/**"],
    jsPlugins: [{ name: "vite-plus", specifier: "vite-plus/oxlint-plugin" }],
    rules: { "vite-plus/prefer-vite-plus-imports": "error" },
    options: { typeAware: true, typeCheck: true },
  },
  run: {
    cache: true,
  },
  tasks: {
    build: {
      command: "node build.mjs",
      cache: {
        input: [{ auto: true }, "!dist/**"],
        output: ["dist/**"],
      },
    },
    check: {
      command: "vp check",
      cache: true,
    },
    test: {
      command: "vp test",
      cache: true,
    },
    ci: {
      command: "tsx src/ci.ts",
      cache: false,
    },
    "ci:check": {
      command: "tsx src/ci.ts check",
      cache: true,
    },
    "ci:build": {
      command: "tsx src/ci.ts build",
      cache: {
        input: [{ auto: true }, "!dist/**"],
        output: ["dist/**"],
      },
    },
    "ci:test": {
      command: "tsx src/ci.ts test",
      cache: true,
    },
  },
});
