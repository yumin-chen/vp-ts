# @lib/ksuid

Fast, native KSUID (K-Sortable Unique Identifier) implementation for Node.js and TypeScript, built with Rust and NAPI-RS.

## What is a KSUID?

KSUID stands for **K-Sortable Unique IDentifier**. It is a globally unique identifier similar to a UUID, designed to be naturally sorted by generation timestamp without any special type-aware logic.

In short, running a set of KSUIDs through the UNIX `sort` command or sorting them lexicographically in JavaScript will result in a list ordered by generation time.

## Installation

```sh
npm install @lib/ksuid
```

## Features

- **High Performance:** Compiled directly to native Node.js addons via NAPI-RS.
- **Timestamp Resolutions:**
  - `Ksuid`: Standard KSUID with a 32-bit timestamp (second-level resolution) + 128-bit payload.
  - `KsuidMs`: High-precision KSUID with a 48-bit timestamp (millisecond-level resolution) + 112-bit payload.
- **Multiple Encodings:**
  - Alphanumeric Base62 (`0ujsswThIGTUYm2K8FjOOfXtY1K`).
  - Base36 (`0372hg16csmsm50l8dikcvukc`).
  - Crockford Base32 with custom/shuffled alphabets and character normalization (`O`/`I`/`L` aliasing).
- **Configurable `toString()`:** Options object `{ enc: "base32" | "base36" | "base62", alphabet?: string }` or global default configuration.

---

## Usage Examples

### Import

```ts
import { Ksuid, KsuidMs, shuffleAlphabet } from "@lib/ksuid";
```

Or in CommonJS:

```js
const { Ksuid, KsuidMs, shuffleAlphabet } = require("@lib/ksuid");
```

---

### Generating and Converting KSUIDs

```ts
// Create Ksuid with current timestamp and random payload
const ksuid = Ksuid.now();

// Base62 String
console.log(ksuid.toString()); // e.g., "1srOrx2ZWZBpBUvZwXKQmoEYga2"
console.log(ksuid.toBase62()); // "1srOrx2ZWZBpBUvZwXKQmoEYga2"

// Base36 String
console.log(ksuid.toBase36()); // e.g., "0372ijojuxuhjsfkeryi2mrtm"

// Crockford Base32 String
console.log(ksuid.toCrockfordBase32()); // e.g., "001E9F1S... (32 chars)"

// Raw 20 Bytes (Uint8Array)
console.log(ksuid.bytes());

// 16-Byte Payload Portion
console.log(ksuid.payloadBytes());

// Timestamp in seconds since UNIX epoch
console.log(ksuid.timestampSeconds()); // 1621627443
```

---

### Creating KSUIDs with Specific Parameters

```ts
// With explicit timestamp (seconds) and 16-byte payload
const payload = new Uint8Array(16);
payload.fill(12);

const customKsuid = Ksuid.fromSeconds(1555555555, payload);

// From Base62 String
const parsedBase62 = Ksuid.fromBase62("1srOrx2ZWZBpBUvZwXKQmoEYga2");

// From Base36 String
const parsedBase36 = Ksuid.fromBase36("0372ijojuxuhjsfkeryi2mrtm");

// From Crockford Base32 String
const parsedCrockford = Ksuid.fromCrockfordBase32("001E9F1S...");

// From 20 Raw Bytes
const parsedBytes = Ksuid.fromBytes(customKsuid.bytes());
```

---

### High Precision KSUID (`KsuidMs` - Millisecond Resolution)

```ts
// KsuidMs uses a 48-bit millisecond timestamp
const ksuidMs = KsuidMs.now();

console.log(ksuidMs.timestampMs()); // e.g., 1718000000123
console.log(ksuidMs.toString()); // Base62 string

// Parse KsuidMs from Base62, Base36, or Crockford Base32
const parsedMs = KsuidMs.fromBase62(ksuidMs.toString());
```

---

### Custom & Shuffled Alphabets

```ts
// Shuffle an alphabet deterministically with a seed
const standardAlpha = "0123456789abcdefghijklmnopqrstuvwxyz";
const shuffledAlpha = shuffleAlphabet(standardAlpha, "my-secret-seed");

const ksuid = Ksuid.now();

// Encode with custom alphabet
const customBase36Str = ksuid.toBase36(shuffledAlpha);

// Decode with custom alphabet
const decoded = Ksuid.fromBase36(customBase36Str, shuffledAlpha);
```

---

### Configurable `toString()` Options

```ts
const ksuid = Ksuid.now();

// Format via options object
console.log(ksuid.toString({ enc: "base36" })); // Base36
console.log(ksuid.toString({ enc: "base32" })); // Crockford Base32
console.log(ksuid.toString({ enc: "base62" })); // Base62

// Format with custom alphabet in options
console.log(ksuid.toString({ enc: "base36", alphabet: shuffledAlpha }));

// Global default configuration for toString()
Ksuid.setDefaultEncoding("base36");
console.log(ksuid.toString()); // Outputs Base36 by default

Ksuid.setDefaultEncoding("base62"); // Reset to Base62
```

---

### Comparing and Ordering KSUIDs

```ts
const ksuid1 = Ksuid.fromSeconds(1555555555);
const ksuid2 = Ksuid.fromSeconds(1777777777);

console.log(ksuid1.compareTo(ksuid2)); // -1
console.log(ksuid2.compareTo(ksuid1)); // 1
console.log(ksuid1.equals(ksuid2)); // false
console.log(ksuid1.equals(ksuid1)); // true
```

## Development & Testing

```sh
# Build native binary and packages
npm run build

# Run unit tests
npm test

# Check formatting and linting
npm run check
```
