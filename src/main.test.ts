import { expect, test } from "vite-plus/test";
import { Ksuid, shuffleAlphabet } from "../index.js";

test("Ksuid now and formatting", () => {
  const ksuid = Ksuid.now();
  expect(ksuid.toString().length).toBeGreaterThan(0);
  expect(ksuid.toBase62()).toBe(ksuid.toString());
  expect(ksuid.bytes().length).toBe(20);
  expect(ksuid.payloadBytes().length).toBe(16);
  expect(ksuid.timestampSeconds()).toBeGreaterThan(1600000000);
});

test("Ksuid creation with explicit payload and timestamp", () => {
  const payload = new Uint8Array(16);
  payload.fill(12);
  const ksuid = Ksuid.fromSeconds(1555555555, payload);
  expect(ksuid.timestampSeconds()).toBe(1555555555);
  expect(ksuid.payloadBytes()).toEqual(payload);
});

test("Ksuid base62 roundtrip", () => {
  const ksuid = Ksuid.now();
  const base62Str = ksuid.toString();
  const parsed = Ksuid.fromBase62(base62Str);
  expect(parsed.toString()).toBe(base62Str);
  expect(parsed.equals(ksuid)).toBe(true);
});

test("Ksuid Crockford Base32 standard roundtrip and aliasing", () => {
  const ksuid = Ksuid.now();
  const crockfordStr = ksuid.toCrockfordBase32();
  expect(crockfordStr.length).toBe(32);

  const parsed = Ksuid.fromCrockfordBase32(crockfordStr);
  expect(parsed.equals(ksuid)).toBe(true);

  // Test Crockford aliasing: 'O'/'o' -> '0', 'I'/'i'/'L'/'l' -> '1'
  const aliasedStr = crockfordStr.replace(/0/g, "O").replace(/1/g, "I");
  const parsedAliased = Ksuid.fromCrockfordBase32(aliasedStr);
  expect(parsedAliased.equals(ksuid)).toBe(true);
});

test("Ksuid Crockford Base32 custom and shuffled alphabets", () => {
  const standardAlpha = "0123456789ABCDEFGHJKMNPQRSTVWXYZ";
  const shuffledAlpha = shuffleAlphabet(standardAlpha, "my-secret-seed");
  expect(shuffledAlpha.length).toBe(32);

  const ksuid = Ksuid.now();
  const customCrockford = ksuid.toCrockfordBase32(shuffledAlpha);
  expect(customCrockford.length).toBe(32);

  const parsedCustom = Ksuid.fromCrockfordBase32(customCrockford, shuffledAlpha);
  expect(parsedCustom.equals(ksuid)).toBe(true);
});

test("Ksuid fromBytes roundtrip", () => {
  const ksuid = Ksuid.now();
  const bytes = ksuid.bytes();
  const parsed = Ksuid.fromBytes(bytes);
  expect(parsed.bytes()).toEqual(bytes);
  expect(parsed.equals(ksuid)).toBe(true);
});

test("Ksuid compare and ordering", () => {
  const ksuid1 = Ksuid.fromSeconds(1555555555);
  const ksuid2 = Ksuid.fromSeconds(1777777777);

  expect(ksuid1.compareTo(ksuid2)).toBe(-1);
  expect(ksuid2.compareTo(ksuid1)).toBe(1);
  expect(ksuid1.compareTo(ksuid1)).toBe(0);
  expect(ksuid1.equals(ksuid1)).toBe(true);
  expect(ksuid1.equals(ksuid2)).toBe(false);
});
