# Local CI Guide

This project supports **100% offline, local CI execution** using Vite+ (`vp run`), `@voidzero-dev/vite-task-client`, and local matrix runner scripts without relying on external cloud services (like GitHub Actions).

---

## 🚀 Quick Start

Run the local CI pipeline:

```bash
# Run native Node.js matrix local CI pipeline
npm run ci

# Or run via Vite Task runner with task caching
npm run ci:local

# Or run local YAML workflow runner parsing ci.yml
npm run ci:yaml
```

---

## 🛠️ Architecture & Frequently Asked Questions

### 1. How does Local CI run without network/cloud?

Instead of uploading code to cloud runners, local CI uses **Vite Task Runner** (`vp run`) and `@voidzero-dev/vite-task-client` to orchestrate jobs locally.
Vite Task Runner provides **Task Caching**: when source code, inputs, and environment variables haven't changed, cached step results are replayed instantly.

### 2. Which runtime should be used for CI builds?

**Node.js** (`node`) is used natively as the job runner engine.

- Node.js executes build scripts (`node build.mjs`), unit tests (`node --test test.cjs` / `vp test`), and WASM target modules natively.
- No heavy Linux virtual machine or Docker daemon is required just to orchestrate CI.

### 3. Do we need a MicroVM or Docker for cross-building Rust binaries?

**No, microVMs are not required.**

- Rust natively supports cross-compilation on host OS using target toolchains such as `cargo-zigbuild` (Zig) and `cargo-xwin` (Windows MSVC headers).
- **WASM compilation (`wasm32-wasip1-threads`)** compiles natively on host and runs directly within Node.js.
- **Docker / QEMU** is purely optional and used only for running non-native Linux binaries in guest CPU architectures. If Docker is unavailable, the local CI script automatically falls back to native host testing.

### 4. Can we still use simple `.yml` job scripts?

**Yes!**
A `ci.yml` file is provided in the repository root. You can run it locally with:

```bash
npm run ci:yaml
```

The script `scripts/run-yaml-ci.mjs` parses `.yml` job configurations (including `uses: voidzero-dev/setup-vp`, matrix definitions, and `run:` steps) and executes them directly on your machine.

---

## 📑 Task Caching with `@voidzero-dev/vite-task-client`

Tasks defined in `vite.config.ts` leverage automatic and fine-grained data tracking:

```ts
tasks: {
  build: {
    command: "node build.mjs --platform",
    cache: {
      input: ["src/**", "Cargo.toml", "Cargo.lock", "build.mjs", "build.rs"],
      output: ["build/**", "dist/**"],
    },
  },
}
```

The `@voidzero-dev/vite-task-client` package allows build scripts to report environment variables and dynamic dependencies directly to the task cache fingerprint.

---

## ⚙️ Triggering Local CI on Git Commit

Local CI checks are integrated into git pre-commit hooks via `vp staged` and `vp check`.
To enable local git hooks:

```bash
npm run prepare
```
