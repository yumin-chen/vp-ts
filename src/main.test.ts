import { expect, test } from "vite-plus/test";
import {
  Ksuid,
  KsuidMs,
  KSUID_BYTES,
  KSUID_PAYLOAD_BYTES,
  KSUID_MS_PAYLOAD_BYTES,
} from "./main.ts";

test("Ksuid constants", () => {
  expect(KSUID_BYTES).toBe(20);
  expect(KSUID_PAYLOAD_BYTES).toBe(16);
  expect(KSUID_MS_PAYLOAD_BYTES).toBe(15);
});

test("Ksuid.now() generates a valid KSUID with default options", () => {
  const ksuid = Ksuid.now();
  expect(ksuid.toBase62()).toHaveLength(27);
  expect(ksuid.toString()).toBe(ksuid.toBase62());
  expect(ksuid.bytes()).toHaveLength(20);
  expect(ksuid.payload()).toHaveLength(16);

  const nowSeconds = Math.floor(Date.now() / 1000);
  expect(Math.abs(ksuid.timestampSeconds() - nowSeconds)).toBeLessThanOrEqual(5);
});

test("Ksuid timestampSize options (32bit, 48bit, 64bit)", () => {
  const tsMs = 1621627443123;
  const tsSec = Math.floor(tsMs / 1000);

  // 32bit (default)
  const k32 = new Ksuid(tsSec, Buffer.alloc(16, 1), { timestampSize: "32bit" });
  expect(k32.payload()).toHaveLength(16);
  expect(k32.timestampSeconds()).toBe(tsSec);

  // 48bit
  const k48 = new Ksuid(tsMs, Buffer.alloc(14, 2), { timestampSize: "48bit" });
  expect(k48.payload()).toHaveLength(14);
  expect(k48.timestampMs()).toBe(tsMs);

  // 64bit
  const k64 = new Ksuid(tsMs, Buffer.alloc(12, 3), { timestampSize: "64bit" });
  expect(k64.payload()).toHaveLength(12);
  expect(k64.timestampMs()).toBe(tsMs);
});

test("Ksuid enc option (base32 vs base62)", () => {
  const kBase62 = Ksuid.now({ enc: "base62" });
  expect(kBase62.toString()).toHaveLength(27);

  const kBase32 = Ksuid.now({ enc: "base32" });
  expect(kBase32.toString()).toHaveLength(32);
  expect(kBase32.toBase32()).toHaveLength(32);

  // Roundtrip base32
  const str32 = kBase32.toBase32();
  const parsed32 = Ksuid.fromBase32(str32);
  expect(parsed32.equals(kBase32)).toBe(true);
});

test("Ksuid custom alphabet option", () => {
  // Shuffled 62-char alphabet
  const shuffled62 = "zyxwvutsrqponmlkjihgfedcbaZYXWVUTSRQPONMLKJIHGFEDCBA9876543210";
  const k = Ksuid.now({ alphabet: shuffled62 });
  const encoded = k.toBase62();
  expect(encoded).toHaveLength(27);

  const parsed = Ksuid.fromBase62(encoded, { alphabet: shuffled62 });
  expect(parsed.equals(k)).toBe(true);

  // Custom 32-char alphabet
  const shuffled32 = "ZYXWVTRPnMkHGFEDCBA9876543210987";
  const k32 = Ksuid.now({ enc: "base32", alphabet: shuffled32 });
  const enc32 = k32.toBase32();
  expect(enc32).toHaveLength(32);
});

test("Ksuid.fromSeconds and new constructor", () => {
  const timestamp = 1621627443;
  const payload = Buffer.alloc(16, 12);

  const ksuid1 = Ksuid.fromSeconds(timestamp, payload);
  expect(ksuid1.timestampSeconds()).toBe(timestamp);
  expect(Buffer.from(ksuid1.payload())).toEqual(payload);

  const ksuid2 = new Ksuid(timestamp, payload);
  expect(ksuid2.equals(ksuid1)).toBe(true);
});

test("Ksuid fromBase62 and fromStr", () => {
  const base62Str = "1srOrx2ZWZBpBUvZwXKQmoEYga2";
  const ksuid = Ksuid.fromBase62(base62Str);
  expect(ksuid.toBase62()).toBe(base62Str);

  const ksuidStr = Ksuid.fromStr(base62Str);
  expect(ksuidStr.equals(ksuid)).toBe(true);

  expect(() => Ksuid.fromBase62("invalid-ksuid!@#$")).toThrow();
});

test("Ksuid compare and equals", () => {
  const k1 = Ksuid.fromSeconds(1555555555, Buffer.alloc(16, 0));
  const k2 = Ksuid.fromSeconds(1777777777, Buffer.alloc(16, 0));
  const k1Copy = Ksuid.fromSeconds(1555555555, Buffer.alloc(16, 0));

  expect(k1.compare(k2)).toBe(-1);
  expect(k2.compare(k1)).toBe(1);
  expect(k1.compare(k1Copy)).toBe(0);

  expect(k1.equals(k1Copy)).toBe(true);
  expect(k1.equals(k2)).toBe(false);
});

test("KsuidMs operations with options", () => {
  const kmNow = KsuidMs.now({ enc: "base32" });
  expect(kmNow.toString()).toHaveLength(32);
  expect(kmNow.bytes()).toHaveLength(20);
  expect(kmNow.payload()).toHaveLength(12);

  const tsMs = 1621627443123;
  const payload = Buffer.alloc(12, 42);
  const km = KsuidMs.fromMillis(tsMs, payload);
  expect(km.timestampMs()).toBe(tsMs);
  expect(Buffer.from(km.payload())).toEqual(payload);
});
