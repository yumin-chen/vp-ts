import { defineConfig } from "vite-plus";

export default defineConfig({
  pack: {
    entry: ["./src/main.ts"],
    format: "esm",
    outDir: "dist",
    exports: true,
    dts: true,
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
  },
  run: {
    cache: true,
  },
  tasks: {
    build: {
      command: "node build.js",
      cache: {
        input: [{ auto: true }, "!dist/**", "!*.node"],
        output: ["dist/**", "*.node"],
        env: ["NODE_ENV", "DEBUG", "MACOSX_DEPLOYMENT_TARGET"],
      },
    },
    check: {
      command: "vp check",
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
    ci: {
      command: "tsx src/ci.ts",
      cache: {
        input: [{ auto: true }, ".github/workflows/*.yml", "src/ci.ts"],
        env: ["DEBUG", "MACOSX_DEPLOYMENT_TARGET"],
      },
    },
    "ci:check": {
      command: "tsx src/ci.ts check",
      cache: {
        input: [{ auto: true }],
      },
    },
    "ci:build": {
      command: "tsx src/ci.ts build",
      cache: {
        input: [{ auto: true }, "!dist/**", "!*.node"],
        output: ["dist/**", "*.node"],
        env: ["DEBUG", "MACOSX_DEPLOYMENT_TARGET"],
      },
    },
    "ci:test": {
      command: "tsx src/ci.ts test",
      cache: {
        input: [{ auto: true }],
      },
    },
    "ci:cross": {
      command: "tsx src/ci.ts cross",
      cache: {
        input: [{ auto: true }],
        output: ["*.node"],
      },
    },
    "ci:publish": {
      command: "tsx src/ci.ts publish",
      cache: {
        input: [{ auto: true }, "!dist/**"],
      },
    },
  },
});
