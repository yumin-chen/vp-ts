# Starter Template

TypeScript starter template with local CI runner and Vite+ task caching.

## Development

- Configure local hooks:

```bash
npm run prepare
```

- Install dependencies:

```bash
vp install
```

- Run unit tests:

```bash
vp test
```

- Run locally:

```bash
npm run dev
```

- Build library:

```bash
npm run build
```

- Cross-build all targets defined in `package.json`:

```bash
node build.mjs --target-all
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

Run local CI jobs offline without relying on cloud services:

```bash
npm run ci
```

For detailed documentation on local CI architecture, cross-build options, and task caching, see [LOCAL_CI_GUIDE.md](./LOCAL_CI_GUIDE.md).
