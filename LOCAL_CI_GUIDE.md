# Local CI Guide

This project supports **100% offline, local CI execution** using Vite+ (`vp run`), `@voidzero-dev/vite-task-client`, and `build.mjs` without relying on external cloud services or YAML runner parsers.

---

## 🚀 Quick Start

Run the local CI pipeline:

```bash
# Run local CI pipeline
npm run ci

# Or run via Vite Task runner with task caching
npm run ci:local
```

### Direct Cross-Build Triggers in `build.mjs`

To run target matrix builds directly with `build.mjs`:

```bash
# Build all targets defined in package.json (napi.targets)
node build.mjs --target-all

# Dry-run target-all cross build
node build.mjs --dry-run --target-all

# Build specific target with cross flags
node build.mjs --target aarch64-unknown-linux-gnu --use-napi-cross

# Release build
node build.mjs --release
```

---

## 🛠️ Architecture & Features

### 1. Dynamic Targets from `package.json`

`build.mjs` reads target triples directly from `napi.targets` in `package.json`, avoiding hardcoded lists in `.yml` files.

### 2. Native CI Runtime without MicroVMs

- **Node.js** executes build scripts, quality checks, and tests natively.
- **Cargo / Rust** cross-compiles targets using native toolchains (`cargo-zigbuild`, `cargo-xwin`, `--use-napi-cross`) on host OS without microVMs or containers.

### 3. Task Caching with `@voidzero-dev/vite-task-client`

Tasks defined in `vite.config.ts` leverage `@voidzero-dev/vite-task-client` and `vp run` cache tracking for automatic input/output file fingerprinting.
