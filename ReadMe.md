# Starter Template

TypeScript and Rust NAPI starter template with offline local CI automation.

## Development

- Configure local hooks:

```bash
npm run prepare
```

- Install dependencies:

```bash
vp install
```

- Run unit and native binding tests:

```bash
npm test
```

- Build local native module and bundle:

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

## Local Offline CI & Cross-Building

This repository supports running CI workflows locally without network or cloud service dependencies.

### Running Local CI Jobs

Run the complete local CI pipeline (format/lint check, cross-build, and test suite):

```bash
npm run ci
```

Or execute individual CI tasks:

```bash
npx vp run ci:check  # Code formatting & lint checks
npx vp run ci:build  # Cross-build target matrix
npx vp run ci:test   # Test suite
```

### Task Caching

Local CI jobs leverage Vite Task Runner and `@voidzero-dev/vite-task-client` for intelligent task caching:

- Task execution results are cached in `node_modules/.vite/task-cache`.
- Re-running `npm run ci` or `npx vp run <task>` will restore outputs and skip unchanged steps instantly.
- Clear task cache if needed:

```bash
npx vp cache clean
```

### Cross-Building Target Matrix

To run target matrix cross-building locally across platforms:

```bash
npm run build:cross
# or
npx vp run build.ts --all
```

To test cross-building with dry run mode:

```bash
npx vp run build.ts --dry-run
```

### YAML Job Script Specification

The local CI pipeline definition is mirrored in `.ci/job.yml`.
