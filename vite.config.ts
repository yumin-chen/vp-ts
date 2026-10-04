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
    "*": ["vp check --fix", "vp test"],
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
    check: {
      command: "vp check",
      cache: {
        input: [{ auto: true }],
      },
    },
    test: {
      command: "vp test",
      cache: {
        input: [{ auto: true }],
      },
    },
    build: {
      command: "node build.ts && vp pack",
      cache: {
        input: [{ auto: true }, "!dist/**", "!target/**"],
        output: ["dist/**"],
      },
    },
    "build:cross": {
      command: "node scripts/cross-build.ts",
      cache: {
        input: [{ auto: true }, "!dist/**", "!target/**"],
        output: ["dist/**"],
      },
    },
    ci: {
      command: "node src/ci.ts",
      cache: {
        input: [{ auto: true }, "!dist/**", "!target/**"],
        output: ["dist/**"],
      },
    },
  },
});
