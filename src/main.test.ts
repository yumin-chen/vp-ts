import { expect, test } from "vite-plus/test";
import {
  Ksuid,
  KsuidMs,
  generateKsuid,
  parseKsuid,
  encodeCrockfordBase32,
  decodeCrockfordBase32,
  encodeBase32BytesJs,
  decodeBase32BytesJs,
  encodeBase36BytesJs,
  decodeBase36BytesJs,
  shuffleAlphabet,
} from "../index.js";
import { main } from "./main.ts";

test("main returns string with Ksuid details", () => {
  const result = main();
  expect(result).toContain("Ksuid (base32 32bit):");
  expect(result).toContain("equals: true");
});

test("Ksuid creation via constructor options", () => {
  const defaultKsuid = new Ksuid();
  expect(defaultKsuid.timestampSize()).toBe("32bit");
  expect(defaultKsuid.enc()).toBe("base62");
  expect(defaultKsuid.alphabet()).toBeNull();
  expect(defaultKsuid.payload()).toHaveLength(16);

  // Configure enc: "base32" and timestampSize: "48bit" / "64bit"
  const stdB32 = "0123456789ABCDEFGHJKMNPQRSTVWXYZ";
  const customB32 = shuffleAlphabet(stdB32, 777);

  const ksuid32B32 = new Ksuid({
    timestamp: 1621627443,
    timestampSize: "32bit",
    enc: "base32",
    alphabet: customB32,
  });

  expect(ksuid32B32.timestampSize()).toBe("32bit");
  expect(ksuid32B32.enc()).toBe("base32");
  expect(ksuid32B32.alphabet()).toBe(customB32);
  expect(ksuid32B32.toString()).toBe(ksuid32B32.toBase32());

  const ksuid48B32 = new Ksuid({
    timestamp: 1621627443000,
    timestampSize: "48bit",
    enc: "base32",
  });

  expect(ksuid48B32.timestampSize()).toBe("48bit");
  expect(ksuid48B32.timestampMilliseconds()).toBe(1621627443000);
  expect(ksuid48B32.payload()).toHaveLength(15);
  expect(ksuid48B32.toString()).toBe(ksuid48B32.toBase32());

  const ksuid64B32 = new Ksuid({
    timestamp: 1621627443000,
    timestampSize: "64bit",
    enc: "base36",
  });

  expect(ksuid64B32.timestampSize()).toBe("64bit");
  expect(ksuid64B32.enc()).toBe("base36");
  expect(ksuid64B32.timestampMilliseconds()).toBe(1621627443000);
  expect(ksuid64B32.toString()).toBe(ksuid64B32.toBase36());
});

test("Ksuid creation and conversion", () => {
  const ksuid = Ksuid.now();
  expect(ksuid.toBase62()).toHaveLength(27);
  expect(ksuid.toString()).toBe(ksuid.toBase62());
  expect(ksuid.bytes()).toHaveLength(20);
  expect(ksuid.payload()).toHaveLength(16);
  expect(ksuid.timestampSeconds()).toBeGreaterThan(0);
});

test("Ksuid with explicit timestamp and payload", () => {
  const payload = new Uint8Array(16);
  payload.fill(12);

  const timestamp = 1621627443;
  const ksuid = Ksuid.fromSeconds(timestamp, payload);
  expect(ksuid.timestampSeconds()).toBe(timestamp);
  expect(ksuid.payload()).toEqual(payload);

  // Roundtrip base62
  const base62 = ksuid.toBase62();
  const parsed = Ksuid.fromBase62(base62);
  expect(parsed.equals(ksuid)).toBe(true);
  expect(parsed.timestampSeconds()).toBe(timestamp);

  // Roundtrip bytes
  const bytes = ksuid.bytes();
  const fromBytes = Ksuid.fromBytes(bytes);
  expect(fromBytes.equals(ksuid)).toBe(true);
});

test("Crockford Base32 and Base36 encoding and decoding", () => {
  const ksuid = Ksuid.now();
  const b32 = ksuid.toBase32();
  expect(b32).toHaveLength(32);

  const parsed = Ksuid.fromBase32(b32);
  expect(parsed.equals(ksuid)).toBe(true);

  // Base36
  const b36 = ksuid.toBase36();
  const parsedB36 = Ksuid.fromBase36(b36);
  expect(parsedB36.equals(ksuid)).toBe(true);

  // Crockford u64
  expect(encodeCrockfordBase32(0)).toBe("0");
  expect(encodeCrockfordBase32(5111)).toBe("4ZQ");
  expect(decodeCrockfordBase32("4ZQ")).toBe(5111);
  expect(decodeCrockfordBase32("4zq")).toBe(5111);

  // Raw bytes Base32 and Base36 helper
  const rawBytes = new Uint8Array([1, 2, 3, 4, 5]);
  const encBytes32 = encodeBase32BytesJs(rawBytes);
  const decBytes32 = decodeBase32BytesJs(encBytes32);
  expect(decBytes32).toEqual(rawBytes);

  const encBytes36 = encodeBase36BytesJs(rawBytes);
  const decBytes36 = decodeBase36BytesJs(encBytes36);
  expect(decBytes36).toEqual(rawBytes);
});

test("Custom and shuffled alphabet configuration", () => {
  const defaultBase32 = "0123456789ABCDEFGHJKMNPQRSTVWXYZ";
  const shuffledBase32 = shuffleAlphabet(defaultBase32, 12345);
  expect(shuffledBase32).toHaveLength(32);
  expect(shuffledBase32).not.toBe(defaultBase32);

  const ksuid = Ksuid.now();
  const customEncoded = ksuid.toBase32(shuffledBase32);
  const parsedCustom = Ksuid.fromBase32(customEncoded, shuffledBase32);
  expect(parsedCustom.equals(ksuid)).toBe(true);

  // Custom Base36 alphabet
  const defaultBase36 = "0123456789abcdefghijklmnopqrstuvwxyz";
  const shuffledBase36 = shuffleAlphabet(defaultBase36, 54321);
  expect(shuffledBase36).toHaveLength(36);

  const b36Custom = ksuid.toBase36(shuffledBase36);
  const parsedB36Custom = Ksuid.fromBase36(b36Custom, shuffledBase36);
  expect(parsedB36Custom.equals(ksuid)).toBe(true);

  // Custom Base62 alphabet
  const defaultBase62 = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
  const shuffledBase62 = shuffleAlphabet(defaultBase62, 99);
  expect(shuffledBase62).toHaveLength(62);

  const b62Custom = ksuid.toBase62(shuffledBase62);
  const parsedB62 = Ksuid.fromBase62(b62Custom, shuffledBase62);
  expect(parsedB62.equals(ksuid)).toBe(true);
});

test("Ksuid comparison and ordering", () => {
  const ksuid1 = Ksuid.fromSeconds(1555555555, null);
  const ksuid2 = Ksuid.fromSeconds(1777777777, null);

  expect(ksuid1.compare(ksuid2)).toBeLessThan(0);
  expect(ksuid2.compare(ksuid1)).toBeGreaterThan(0);
  expect(ksuid1.compare(ksuid1)).toBe(0);
  expect(ksuid1.equals(ksuid1)).toBe(true);
  expect(ksuid1.equals(ksuid2)).toBe(false);
});

test("KsuidMs creation and conversion", () => {
  const payload = new Uint8Array(15);
  payload.fill(7);

  const timestampMs = 1621627443000;
  const ksuidMs = KsuidMs.fromMilliseconds(timestampMs, payload);
  expect(ksuidMs.timestampMilliseconds()).toBe(timestampMs);
  expect(ksuidMs.payload()).toEqual(payload);
  expect(ksuidMs.bytes()).toHaveLength(20);

  const base62 = ksuidMs.toBase62();
  const parsed = KsuidMs.fromBase62(base62);
  expect(parsed.equals(ksuidMs)).toBe(true);
});

test("Standalone helper functions", () => {
  const base62 = generateKsuid();
  expect(base62).toHaveLength(27);

  const parsed = parseKsuid(base62);
  expect(parsed.toBase62()).toBe(base62);
});
