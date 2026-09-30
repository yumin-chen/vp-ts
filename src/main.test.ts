import { expect, test } from "vite-plus/test";
import { Ksuid, KsuidMs, shuffleAlphabet } from "../index.js";

test("Ksuid now and formatting", () => {
  Ksuid.setDefaultEncoding("base62");
  const ksuid = Ksuid.now();
  expect(ksuid.toString().length).toBeGreaterThan(0);
  expect(ksuid.toBase62()).toBe(ksuid.toString());
  expect(ksuid.bytes().length).toBe(20);
  expect(ksuid.payloadBytes().length).toBe(16);
  expect(ksuid.timestampSeconds()).toBeGreaterThan(1600000000);
  expect(ksuid.timestampMs()).toBe(ksuid.timestampSeconds() * 1000);
});

test("KsuidMs 48-bit millisecond resolution timestamp precision", () => {
  const ksuidMs = KsuidMs.now();
  const tsMs = ksuidMs.timestampMs();
  expect(tsMs).toBeGreaterThan(1600000000000);
  expect(ksuidMs.bytes().length).toBe(20);

  const base62Str = ksuidMs.toString();
  const parsed = KsuidMs.fromBase62(base62Str);
  expect(parsed.timestampMs()).toBe(tsMs);
  expect(parsed.equals(ksuidMs)).toBe(true);
});

test("Ksuid creation with explicit payload and timestamp", () => {
  const payload = new Uint8Array(16);
  payload.fill(12);
  const ksuid = Ksuid.fromSeconds(1555555555, payload);
  expect(ksuid.timestampSeconds()).toBe(1555555555);
  expect(ksuid.payloadBytes()).toEqual(payload);
});

test("Ksuid base62 roundtrip", () => {
  Ksuid.setDefaultEncoding("base62");
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

test("Ksuid toString with options object ({ enc: 'base32' / 'base62' })", () => {
  Ksuid.setDefaultEncoding("base62");
  const ksuid = Ksuid.now();

  // Test enc option
  expect(ksuid.toString({ enc: "base32" })).toBe(ksuid.toCrockfordBase32());
  expect(ksuid.toString({ enc: "crockford" })).toBe(ksuid.toCrockfordBase32());
  expect(ksuid.toString({ enc: "base62" })).toBe(ksuid.toBase62());

  // Test with custom alphabet in options object
  const customAlpha = shuffleAlphabet("0123456789ABCDEFGHJKMNPQRSTVWXYZ", "opt-seed");
  expect(ksuid.toString({ enc: "base32", alphabet: customAlpha })).toBe(
    ksuid.toCrockfordBase32(customAlpha),
  );
});

test("Ksuid configurable default toString encoding", () => {
  const ksuid = Ksuid.now();

  // Configure global default to Crockford
  Ksuid.setDefaultEncoding("crockford");
  expect(Ksuid.getDefaultEncoding()).toBe("crockford");
  expect(ksuid.toString()).toBe(ksuid.toCrockfordBase32());

  // Configure global default with custom shuffled alphabet
  const customAlpha = shuffleAlphabet("0123456789ABCDEFGHJKMNPQRSTVWXYZ", "custom-seed");
  Ksuid.setDefaultEncoding("crockford", customAlpha);
  expect(ksuid.toString()).toBe(ksuid.toCrockfordBase32(customAlpha));

  // Reset back to base62
  Ksuid.setDefaultEncoding("base62");
  expect(Ksuid.getDefaultEncoding()).toBe("base62");
  expect(ksuid.toString()).toBe(ksuid.toBase62());
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
