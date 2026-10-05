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
    ignorePatterns: ["examples/**", "dist/**"],
  },
  lint: {
    ignorePatterns: ["examples/**", "dist/**"],
    jsPlugins: [{ name: "vite-plus", specifier: "vite-plus/oxlint-plugin" }],
    rules: { "vite-plus/prefer-vite-plus-imports": "error" },
    options: { typeAware: true, typeCheck: true },
  },
  test: {
    include: ["src/**/*.test.ts"],
    exclude: ["examples/**", "dist/**"],
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
        output: ["dist/**"],
      },
    },
    test: {
      command: "vp test",
    },
    ci: {
      command: "node scripts/ci.mjs",
    },
    "publish-dry-run": {
      command: "node scripts/publish-dry-run.mjs",
    },
  },
});
