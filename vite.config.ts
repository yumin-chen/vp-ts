import { defineConfig } from "vite-plus";

export default defineConfig({
  create: {
    defaultTemplate: "@yumin-chen",
    templates: [
      {
        name: "vp-ts",
        description: "TypeScript starter template",
        template: "./templates/vp-ts",
      },
    ],
  },
  pack: {
    entry: ["./src/main.ts"],
    format: "esm",
    outDir: "dist",
    dts: {
      generator: "tsgo",
    },
    exports: true,
    deps: {
      // tsdown <0.23 compatibility: resolve external dependency subpaths.
      // Remove to preserve subpath imports as written (the new default).
      // https://tsdown.dev/options/dependencies#deps-resolvedepsubpath
      resolveDepSubpath: true,
    },
  },
});
