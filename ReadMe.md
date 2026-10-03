# @lib/git2

Node.js NAPI bindings for Git operations, backed by `gix` (gitoxide) in Rust with `git2` API compatibility.

## Usage

```javascript
const { Repository, Signature, Reference } = require("@lib/git2");

// Initialize or open a repository
const repo = Repository.init("/path/to/repo");
console.log("Is bare:", repo.isBare());
console.log("Is empty:", repo.isEmpty());

// Signatures
const sig = Signature.now("User Name", "user@example.com");

// Working with References
if (Reference.isValidName("refs/heads/main")) {
  console.log("Valid reference name");
}
```

## Development

- Install dependencies:

```bash
vp install
```

- Run unit tests:

```bash
npm test
```

- Build release binary:

```bash
npm run build:release
```

- Code formatting:

```bash
npm run fmt
```
