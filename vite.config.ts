import { defineConfig } from "vite-plus";

export default defineConfig({
  pack: {
    entry: ["./src/main.ts"],
    format: "esm",
    outDir: "dist",
    exports: true,
    dts: {
      generator: "oxc",
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
    build: {
      command: "node build.mjs && vp pack",
      cache: {
        input: [{ auto: true }, "!dist/**", "!npm/**"],
        output: ["dist/**", "*.node", "npm/**"],
        env: ["NODE_ENV"],
      },
    },
    check: {
      command: "vp check src/",
      cache: {
        input: [{ auto: true }],
      },
    },
    test: {
      command: "node --test test.cjs && vp test",
      cache: {
        input: [{ auto: true }],
        env: ["NODE_ENV"],
      },
    },
    "cross-build": {
      command: "node build.mjs --use-cross",
      cache: {
        input: [{ auto: true }, "!dist/**", "!npm/**"],
        output: ["npm/**", "*.node"],
      },
    },
    ci: {
      command: "node --experimental-strip-types src/ci.ts",
      cache: {
        input: [{ auto: true }],
        env: ["CI", "NODE_ENV"],
      },
    },
    "publish:local": {
      command: "npm run build",
      cache: {
        input: [{ auto: true }],
        output: ["dist/**"],
      },
    },
  },
});
