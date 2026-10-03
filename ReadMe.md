# Starter Template

TypeScript starter template with local CI job runner and Vite+ task caching.

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

- Run locally:

```bash
npm run dev
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

## Local CI Pipeline

Run local CI jobs completely offline without relying on cloud services:

- **Run Native Local CI Matrix Pipeline:**

```bash
npm run ci
```

- **Run Local CI via Vite Task Runner (with Task Caching):**

```bash
npm run ci:local
```

- **Run YAML Workflow Script (`ci.yml`):**

```bash
npm run ci:yaml
```

For complete documentation on local CI architecture, task caching, and cross-building, see [LOCAL_CI_GUIDE.md](./LOCAL_CI_GUIDE.md).
