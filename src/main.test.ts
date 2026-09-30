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

test("Ksuid.now() generates a valid KSUID", () => {
  const ksuid = Ksuid.now();
  expect(ksuid.toBase62()).toHaveLength(27);
  expect(ksuid.toString()).toBe(ksuid.toBase62());
  expect(ksuid.bytes()).toHaveLength(20);
  expect(ksuid.payload()).toHaveLength(16);

  const nowSeconds = Math.floor(Date.now() / 1000);
  expect(Math.abs(ksuid.timestampSeconds() - nowSeconds)).toBeLessThanOrEqual(5);
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

test("Ksuid fromBytes", () => {
  const bytes = Buffer.alloc(20, 7);
  const ksuid = Ksuid.fromBytes(bytes);
  expect(Buffer.from(ksuid.bytes())).toEqual(bytes);

  expect(() => Ksuid.fromBytes(Buffer.alloc(10))).toThrow();
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

test("KsuidMs operations", () => {
  const kmNow = KsuidMs.now();
  expect(kmNow.toBase62()).toHaveLength(27);
  expect(kmNow.bytes()).toHaveLength(20);
  expect(kmNow.payload()).toHaveLength(15);

  const tsMs = 1621627443000;
  const payload = Buffer.alloc(15, 42);
  const km = KsuidMs.fromMillis(tsMs, payload);
  expect(km.timestampMs()).toBe(tsMs);
  expect(Buffer.from(km.payload())).toEqual(payload);

  const base62 = km.toBase62();
  const kmFromBase62 = KsuidMs.fromBase62(base62);
  expect(kmFromBase62.equals(km)).toBe(true);

  const bytes = Buffer.alloc(20, 9);
  const kmFromBytes = KsuidMs.fromBytes(bytes);
  expect(Buffer.from(kmFromBytes.bytes())).toEqual(bytes);
  expect(() => KsuidMs.fromBytes(Buffer.alloc(10))).toThrow();
});
