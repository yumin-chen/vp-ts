# Local CI & Cross-Build Pipeline Starter

TypeScript & NAPI-RS Native Addon Starter with fully offline Local CI orchestration and task caching.

## Features & Architecture

- **100% Offline Local CI**: Run all CI stages locally without relying on cloud workflows, GitHub Actions, or external network services.
- **Native Task Orchestration & Caching**: Powered by Vite+ (`vp run`) and `@voidzero-dev/vite-task-client` for intelligent task caching. Output files and stdout/stderr are cached and instantly replayed on cache hits.
- **Native Cross-Building**: Matrix cross-compilation powered by `@napi-rs/cli`, `cargo-zigbuild`, and `cargo-xwin` directly on host platforms without requiring heavy Docker containers or Linux kernel microVMs.
- **Pre-Commit Hook Integration**: Pre-commit automated code checks and testing configured via `vite.config.ts` (`staged`).
- **Declarative YAML Workflow Support**: `.github/workflows/local-ci.yml` included for YAML-compatible local runner tools (e.g. `act`).

---

## Quick Start

### 1. Install Dependencies & Local Hooks

```bash
npm install
npm run prepare
```

### 2. Local Development & Testing

- **Run Code Checks & Formatting:**
  ```bash
  npm run check
  ```
- **Run Unit Tests:**
  ```bash
  npm run test
  ```
- **Build Native Addon & Bundle Package:**
  ```bash
  npm run build
  ```

---

## Local CI Execution

### Run Full Local CI Pipeline

Run all CI stages (code check, unit tests, host native build, matrix cross-build dry-run, package verification) locally:

```bash
npm run ci
```

Alternatively, invoke the TypeScript CI runner directly:

```bash
npm run ci:ts
```

### Local Cross-Build Matrix

Build or dry-run matrix targets specified in `package.json` (`napi.targets`):

- **Dry-run all matrix targets:**

  ```bash
  npm run ci:cross -- --all --dry-run
  ```

- **Build a specific target:**

  ```bash
  npm run ci:cross -- --target x86_64-unknown-linux-gnu --use-napi-cross
  ```

- **Build all matrix targets:**
  ```bash
  npm run ci:cross -- --all
  ```

---

## Vite Task Caching & Config

Task caching is configured in `vite.config.ts`:

```ts
tasks: {
  check: { command: "vp check", cache: { input: [{ auto: true }] } },
  test: { command: "vp test", cache: { input: [{ auto: true }] } },
  build: {
    command: "node build.ts && vp pack",
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
}
```

Clear the task cache anytime:

```bash
npx vp cache clean
```
