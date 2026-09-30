import { expect, test } from "vite-plus/test";
import {
  Ksuid,
  generateKsuid,
  getKsuidTime,
  encodeCrockford,
  decodeCrockford,
  convertToCrockford,
  convertFromCrockford,
} from "./main.ts";

test("generateKsuid returns a 27-character KSUID", () => {
  const ksuid = generateKsuid();
  expect(ksuid).toHaveLength(27);
  expect(getKsuidTime(ksuid)).toBeGreaterThan(1600000000);
});

test("crockford encoding and custom alphabet shuffle", () => {
  expect(encodeCrockford(65535)).toBe("1ZZZ");
  expect(decodeCrockford("1zzz")).toBe(65535);

  const customAlphabet = "ZYXWVUTSRQPONMKJHGFEDCBA98765432";
  const customEncoded = encodeCrockford(65535, customAlphabet);
  expect(decodeCrockford(customEncoded, customAlphabet)).toBe(65535);

  const ksuid = generateKsuid();
  const crockford = convertToCrockford(ksuid, customAlphabet);
  expect(convertFromCrockford(crockford, customAlphabet)).toBe(ksuid);
});

test("Ksuid class options with enc base32 and timestampSize options", () => {
  const customAlphabet = "FxnXM1kBN6cuhsAvjW3Co7l2RePyY8DwaU04Tzt9fHQrqSVKdpimLGIJOgb5ZE";
  const client32 = new Ksuid({
    alphabet: customAlphabet,
    enc: "base32",
    timestampSize: "32bit",
  });
  expect(client32.toString().length).toBeGreaterThan(0);
  expect(client32.timestampSeconds()).toBeGreaterThan(1600000000);

  const client48 = new Ksuid({
    enc: "crockford",
    timestampSize: "48bit",
  });
  expect(client48.toString().length).toBeGreaterThan(0);
  expect(client48.timestampMillis()).toBeGreaterThan(1600000000000);

  const client64 = new Ksuid({
    enc: "base32",
    timestampSize: "64bit",
  });
  expect(client64.toString().length).toBeGreaterThan(0);
  expect(client64.timestampMillis()).toBeGreaterThan(1600000000000);
});
