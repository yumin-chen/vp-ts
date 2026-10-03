# Starter Template

TypeScript starter template with local CI runner, task caching, and native cross-building.

## Development

- Configure local hooks:

```bash
npm run prepare
```

- Install dependencies:

```bash
vp install
```

- Run the unit tests:

```bash
vp test
```

- Build the library:

```bash
npm run build
```

- Code formatting:

```bash
npm run fmt
```

- Linting:

```bash
npm run lint
```

- Code check:

```bash
npm run check
```

## Local CI & Task Caching

This repository includes local CI jobs managed by Vite+ (`vp run`) and `@voidzero-dev/vite-task-client` for task caching.

### Running Local CI Jobs

To run the complete local CI suite (formatting check, build, tests, prepublish packing):

```bash
npm run ci
# or
vp run ci
```

### Running TypeScript Local CI Orchestrator

You can also run the full local CI and publish dry-run pipeline written directly in TypeScript (`src/ci.ts`):

```bash
npm run ci:ts
```

### Pre-Commit and Post-Commit Hooks

- **Pre-commit**: Runs fast code formatting and lint check (`ci:check`):
  ```bash
  npm run precommit
  ```
- **Post-commit**: Runs host build, test suite, and prepublish packing (`ci:build && ci:test && ci:prepublish`):
  ```bash
  npm run postcommit
  ```

### Publishing Workflow

To run cross-building for all targets, pack artifacts, and execute a publish dry-run directly from prepublished cache:

```bash
npm run ci:publish
```

### Native Cross-Building

Cross-compilation is configured directly in `build.mjs` using `@napi-rs/cli` (`createBuildCommand`) and reads targets dynamically from `package.json` (`napi.targets`). It uses `cargo-zigbuild`, `cargo-xwin`, and `napi-cross` flags natively without requiring Docker containers or Linux microVMs:

```bash
# Dry run cross-building all targets declared in package.json
node build.mjs --use-cross --dry-run
# or
npm run ci:cross-build -- --dry-run

# Build a specific target
node build.mjs --target x86_64-unknown-linux-gnu
```
