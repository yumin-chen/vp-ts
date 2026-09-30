import { expect, test } from "vite-plus/test";
import { Ksuid, KsuidMs, generateKsuid, generateKsuidMs } from "./main.ts";

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

test("Helper generator functions", () => {
  const str1 = generateKsuid();
  const str2 = generateKsuidMs();
  expect(typeof str1).toBe("string");
  expect(typeof str2).toBe("string");
  expect(str1.length).toBe(27);
  expect(str2.length).toBe(27);
});
