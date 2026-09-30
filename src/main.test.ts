import { expect, test } from "vite-plus/test";
import { CrockfordBase32, Ksuid, KsuidMs, generateKsuid, generateKsuidMs } from "./main.ts";

test("Ksuid - creation and base62 conversion", () => {
  const ksuid = Ksuid.now();
  expect(ksuid.toBase62().length).toBe(27);
  expect(ksuid.toString()).toBe(ksuid.toBase62());
});

test("Ksuid - bytes and payload lengths", () => {
  const ksuid = Ksuid.now();
  const bytes = ksuid.bytes();
  const payload = ksuid.payload();

  expect(bytes.length).toBe(Ksuid.BYTE_SIZE);
  expect(payload.length).toBe(Ksuid.PAYLOAD_BYTES);
});

test("Ksuid - payload explicit bytes", () => {
  const payloadBytes = new Uint8Array(16).fill(12);
  const ksuid = Ksuid.new(1621627443, payloadBytes);

  expect(ksuid.timestampSeconds()).toBe(1621627443);
  expect(Array.from(ksuid.payload())).toEqual(Array.from(payloadBytes));
});

test("Ksuid - fromBase62 and constructor with string", () => {
  const base62 = "1srOrx2ZWZBpBUvZwXKQmoEYga2";
  const ksuid1 = Ksuid.fromBase62(base62);
  const ksuid2 = new Ksuid(base62);

  expect(ksuid1.toBase62()).toBe(base62);
  expect(ksuid2.toBase62()).toBe(base62);
  expect(ksuid1.equals(ksuid2)).toBe(true);
});

test("Ksuid - fromBytes and constructor with bytes", () => {
  const bytes = new Uint8Array(20).fill(7);
  const ksuid1 = Ksuid.fromBytes(bytes);
  const ksuid2 = new Ksuid(bytes);

  expect(Array.from(ksuid1.bytes())).toEqual(Array.from(bytes));
  expect(ksuid1.equals(ksuid2)).toBe(true);
});

test("Ksuid - comparison and ordering", () => {
  const ksuid1 = Ksuid.fromSeconds(1_555_555_555);
  const ksuid2 = Ksuid.fromSeconds(1_777_777_777);

  expect(ksuid1.compare(ksuid2)).toBeLessThan(0);
  expect(ksuid2.compare(ksuid1)).toBeGreaterThan(0);
  expect(ksuid1.compare(ksuid1)).toBe(0);
  expect(ksuid1.equals(ksuid1)).toBe(true);
  expect(ksuid1.equals(ksuid2)).toBe(false);
});

test("KsuidMs - creation and timestamp", () => {
  const msKsuid = KsuidMs.fromMilliseconds(1_621_627_443_000);
  expect(msKsuid.timestampMilliseconds()).toBe(1_621_627_443_000);
  expect(msKsuid.toBase62().length).toBe(27);
});

test("CrockfordBase32 - u64 encoding and decoding", () => {
  const cb32 = CrockfordBase32.defaultEncoder();
  expect(cb32.encode(0)).toBe("0");
  expect(cb32.encode(5111)).toBe("4ZQ");
  expect(cb32.decodeU64("4ZQ")).toBe(5111);
  expect(cb32.decodeU64("4zq")).toBe(5111);
});

test("CrockfordBase32 - custom alphabet and shuffling", () => {
  const defaultAlpha = "0123456789ABCDEFGHJKMNPQRSTVWXYZ";
  const cb32Default = CrockfordBase32.defaultEncoder();
  expect(cb32Default.getAlphabet()).toBe(defaultAlpha);

  const customAlpha = "ZYXWVUTSRQPNMKJHGFEDCBA987654321";
  const cb32Custom = CrockfordBase32.withAlphabet(customAlpha);
  expect(cb32Custom.getAlphabet()).toBe(customAlpha);

  const shuffled = cb32Default.shuffle("secret-seed");
  expect(shuffled.getAlphabet().length).toBe(32);
  expect(shuffled.getAlphabet()).not.toBe(defaultAlpha);

  const encoded = shuffled.encode(1234567);
  const decoded = shuffled.decodeU64(encoded);
  expect(decoded).toBe(1234567);
});

test("Ksuid - Crockford Base32 encoding & roundtrip", () => {
  const ksuid = Ksuid.now();
  const c32Str = ksuid.toCrockfordBase32();
  expect(c32Str.length).toBe(32);

  const restored = Ksuid.fromCrockfordBase32(c32Str);
  expect(restored.equals(ksuid)).toBe(true);

  // Custom shuffled Crockford Base32 for Ksuid
  const encoder = CrockfordBase32.defaultEncoder().shuffle("my-secret");
  const customC32 = ksuid.toCrockfordBase32(encoder);
  const restoredCustom = Ksuid.fromCrockfordBase32(customC32, encoder);
  expect(restoredCustom.equals(ksuid)).toBe(true);
});

test("Helper generator functions", () => {
  const str1 = generateKsuid();
  const str2 = generateKsuidMs();
  expect(typeof str1).toBe("string");
  expect(typeof str2).toBe("string");
  expect(str1.length).toBe(27);
  expect(str2.length).toBe(27);
});
