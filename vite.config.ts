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
    "*": "vp check",
  },
  fmt: {
    ignore: ["examples/**"],
  },
  lint: {
    ignorePatterns: ["examples/**"],
    jsPlugins: [{ name: "vite-plus", specifier: "vite-plus/oxlint-plugin" }],
    rules: { "vite-plus/prefer-vite-plus-imports": "error" },
    options: { typeAware: false, typeCheck: false },
  },
  run: {
    cache: {
      tasks: true,
      scripts: true,
    },
  },
  tasks: {
    "ci:check": {
      command: "vp check",
    },
    "ci:build": {
      command: "node build.js --all",
      cache: {
        input: [{ auto: true }, "!dist/**", "!build/**"],
        output: ["build/**"],
        untrackedEnv: ["CI", "DEBUG"],
      },
    },
    "ci:test": {
      command: "npm test",
    },
  },
});
