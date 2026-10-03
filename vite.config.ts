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
    options: { typeAware: true, typeCheck: true },
  },
  test: {
    include: ["src/**/*.test.ts"],
    exclude: ["examples/**"],
  },
  run: {
    cache: true,
  },
  tasks: {
    check: {
      command: "vp check",
    },
    build: {
      command: "node build.mjs --platform",
      cache: {
        input: ["src/**", "Cargo.toml", "Cargo.lock", "build.mjs", "build.rs"],
        output: ["build/**", "dist/**"],
      },
    },
    test: {
      command: "node --test test.cjs",
    },
    ci: {
      command: "node scripts/ci.mjs",
    },
    "publish-dry-run": {
      command: "node scripts/publish-dry-run.mjs",
    },
  },
});
