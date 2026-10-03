# @lib/git2

Node.js native bindings for `git2` (0.21.0) built with NAPI-RS and Vite Plus (`vp`).

## Overview

`@lib/git2` provides fast, type-safe Node.js bindings to the `git2` Rust library (libgit2).

## Installation

```bash
npm install @lib/git2
```

## Usage

### NAPI Bindings

```typescript
import { Repository, Signature } from "@lib/git2";

// Initialize or open a repository
const repo = Repository.init("./my-repo");
console.log("Is bare:", repo.isBare());
console.log("Path:", repo.path());

// Access objects and references
const head = repo.head();
console.log("HEAD name:", head.name());

// Signatures
const sig = new Signature("User Name", "user@example.com");
console.log("Author:", sig.name(), sig.email());
```

### CLI Subcommands

```typescript
import { cliInit, cliClone, cliStatus, cliMain } from "@lib/git2";

// CLI init
cliInit("./new-repo", false);

// CLI clone
cliClone("https://github.com/example/repo.git", "./cloned-repo");

// CLI status
const status = cliStatus("./new-repo");

// CLI main handler
cliMain(["status", "./new-repo"]);
```

## Development

- Install dependencies:

```bash
vp install
```

- Build native bindings and TypeScript declarations:

```bash
npm run build
```

- Run tests:

```bash
npm test
```

- Code formatting & linting:

```bash
npm run fmt
npm run lint
```
