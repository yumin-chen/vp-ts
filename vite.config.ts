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
  staged: {
    "*": "vp check --fix",
  },
  fmt: {
    ignorePatterns: ["examples/**"],
  },
  lint: {
    ignorePatterns: ["examples/**"],
    jsPlugins: [{ name: "vite-plus", specifier: "vite-plus/oxlint-plugin" }],
    rules: { "vite-plus/prefer-vite-plus-imports": "error" },
    options: { typeAware: true, typeCheck: false },
  },
  run: {
    cache: true,
  },
  tasks: {
    build: {
      command: "node build.mjs",
      cache: {
        input: [{ auto: true }, "!dist/**", "!npm/**"],
        output: ["dist/**"],
      },
    },
    check: {
      command: "tsx src/ci.ts check",
      cache: {
        input: [{ auto: true }],
      },
    },
    test: {
      command: "npm test",
      cache: {
        input: [{ auto: true }],
      },
    },
    prepublish: {
      command: "tsx src/ci.ts prepublish",
      cache: {
        input: [{ auto: true }],
        output: ["npm/**"],
      },
    },
    publish: {
      command: "tsx src/ci.ts publish",
      cache: false,
    },
    ci: {
      command: "tsx src/ci.ts all",
      cache: {
        input: [{ auto: true }],
        output: ["dist/**", "npm/**"],
      },
    },
  },
});
