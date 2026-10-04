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
    exclude: ["examples/**", "node_modules/**"],
  },
  staged: {
    "*": "vp check --fix",
  },
  fmt: {},
  lint: {
    jsPlugins: [{ name: "vite-plus", specifier: "vite-plus/oxlint-plugin" }],
    rules: { "vite-plus/prefer-vite-plus-imports": "error" },
    options: { typeAware: true, typeCheck: true },
  },
  run: {
    cache: true,
  },
  tasks: {
    ci: {
      command: "node scripts/cross-build.mjs && vp check && vp test",
      cache: {
        input: [{ auto: true }, "!dist/**", "!npm/**"],
        output: ["dist/**"],
      },
    },
    "ci:cross": {
      command: "node scripts/cross-build.mjs --all",
      cache: {
        input: ["src/**", "Cargo.toml", "Cargo.lock"],
        output: ["npm/**"],
      },
    },
  },
});
