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

To run the complete local CI suite (formatting check, build, tests, cross-build dry run):

```bash
npm run ci
# or
vp run ci
```

### Running TypeScript Local CI Orchestrator

You can also run the local CI pipeline written directly in TypeScript (`src/ci.ts`):

```bash
npm run ci:ts
```

### Native Cross-Building

Cross-compilation is configured directly in `build.js` and reads targets dynamically from `package.json` (`napi.targets`). It uses `cargo-zigbuild`, `cargo-xwin`, and `napi-cross` flags natively without requiring Docker containers or Linux microVMs:

```bash
# Dry run cross-building all targets declared in package.json
node build.js --use-cross --dry-run
# or
npm run ci:cross-build -- --dry-run

# Build a specific target
node build.js --target x86_64-unknown-linux-gnu
```

### Pre-Commit Trigger

Pre-commit checks can be run manually or triggered via git hooks:

```bash
npm run precommit
```
