# @lib/ksuid

A high-performance Node.js native addon for KSUID (K-Sortable Unique Identifier) powered by Rust (`svix-ksuid` and `napi-rs`).

## Features

- **KSUID Generation & Parsing**: Fast native generation and parsing of standard KSUID (seconds resolution) and KSUID-Ms (millisecond resolution).
- **Flexible Encodings**: Supports **Base62**, **Crockford Base32**, and **Base36** encodings.
- **Configurable Constructor Options**:
  - `timestampSize`: `"32bit"` (4 bytes, seconds resolution), `"48bit"` (6 bytes, millisecond resolution), or `"64bit"` (8 bytes, millisecond resolution).
  - `enc`: Default string encoding output for `toString()` (`"base62"` (default), `"base32"` / Crockford, or `"base36"`).
  - `alphabet`: Custom or shuffled alphabet configuration for encoding and decoding.
- **Alphabet Shuffling**: Deterministic `shuffleAlphabet(alphabet, seed)` utility.

## Installation

```bash
npm install
```

## Quick Start

```typescript
import { Ksuid, KsuidMs, generateKsuid, parseKsuid, shuffleAlphabet } from "@lib/ksuid";

// Generate standard KSUID
const ksuid = Ksuid.now();
console.log(ksuid.toBase62()); // e.g. "2X5BWE7MY711DAVM00KCMYYA4WVA42S7"

// Crockford Base32 encoding with custom timestampSize
const ksuidB32 = new Ksuid({
  timestampSize: "48bit",
  enc: "base32",
});
console.log(ksuidB32.toString()); // Crockford Base32 string

// Custom shuffled alphabet
const defaultB32 = "0123456789ABCDEFGHJKMNPQRSTVWXYZ";
const shuffledB32 = shuffleAlphabet(defaultB32, 42);

const ksuidCustom = new Ksuid({
  enc: "base32",
  alphabet: shuffledB32,
});
console.log(ksuidCustom.toString());
```

## Constructor & Options

```typescript
const ksuid = new Ksuid({
  timestamp?: number,           // Explicit timestamp (seconds or ms depending on timestampSize)
  payload?: Uint8Array,         // Explicit payload (16 bytes for 32bit, 15 bytes for 48bit/64bit)
  timestampSize?: "32bit" | "48bit" | "64bit", // Default "32bit"
  enc?: "base62" | "base32" | "base36",        // Default "base62"
  alphabet?: string,            // Custom encoding alphabet
});
```

## API Reference

### `Ksuid` Class
- `constructor(options?: KsuidOptions)`
- `static now(options?: KsuidOptions): Ksuid`
- `static new(options?: KsuidOptions): Ksuid`
- `static fromSeconds(seconds?: number, payload?: Uint8Array): Ksuid`
- `static fromMilliseconds(ms?: number, payload?: Uint8Array): Ksuid`
- `static fromBase62(base62: string, alphabet?: string, timestampSize?: string): Ksuid`
- `static fromBase32(encoded: string, alphabet?: string, timestampSize?: string): Ksuid`
- `static fromBase36(encoded: string, alphabet?: string, timestampSize?: string): Ksuid`
- `static fromBytes(bytes: Uint8Array, timestampSize?: string): Ksuid`
- `toBase62(alphabet?: string): string`
- `toBase32(alphabet?: string): string`
- `toBase36(alphabet?: string): string`
- `toString(): string`
- `bytes(): Uint8Array`
- `payload(): Uint8Array`
- `timestampSeconds(): number`
- `timestampMilliseconds(): number`
- `timestampSize(): string`
- `enc(): string`
- `alphabet(): string | null`
- `compare(other: Ksuid): number`
- `equals(other: Ksuid): boolean`

### Standalone Helpers
- `encodeCrockfordBase32(n: number, alphabet?: string): string`
- `decodeCrockfordBase32(encoded: string, alphabet?: string): number`
- `encodeBase32BytesJs(bytes: Uint8Array, alphabet?: string): string`
- `decodeBase32BytesJs(encoded: string, alphabet?: string): Uint8Array`
- `encodeBase36BytesJs(bytes: Uint8Array, alphabet?: string): string`
- `decodeBase36BytesJs(encoded: string, alphabet?: string): Uint8Array`
- `shuffleAlphabet(alphabet: string, seed?: number): string`
- `generateKsuid(alphabet?: string): string`
- `parseKsuid(encoded: string, alphabet?: string): Ksuid`

## Development & Testing

- **Build Addon**: `npm run build`
- **Run Tests**: `npm test`
- **Lint & Check**: `npm run check`
