import { defineConfig } from "vite-plus";

export default defineConfig({
  pack: {
    entry: ["./src/main.ts"],
    format: "esm",
    outDir: "dist",
    exports: true,
    dts: false,
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
    "ci:build": {
      command: "npm run build",
      cache: {
        input: [
          "src/**/*.rs",
          "src/**/*.ts",
          "Cargo.toml",
          "Cargo.lock",
          "package.json",
          "build.js",
        ],
        output: ["build/**", "dist/**"],
      },
    },
    "ci:check": {
      command: "vp check",
      cache: {
        input: ["src/**", "package.json", "tsconfig.json", "vite.config.ts"],
      },
    },
    "ci:test": {
      command: "npm test",
      cache: {
        input: ["src/**", "build/**", "test.cjs"],
      },
    },
    "ci:cross-build": {
      command: "node build.js --use-napi-cross",
      cache: {
        input: ["src/**/*.rs", "Cargo.toml", "Cargo.lock"],
        output: ["build/**"],
      },
    },
    "ci:publish": {
      command: "vp run ci:build && vp run ci:check && vp run ci:test",
      cache: false,
    },
    "ci:local": {
      command: "node .ci/local-ci.mjs",
      cache: false,
    },
  },
});
