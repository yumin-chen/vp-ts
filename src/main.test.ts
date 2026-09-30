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

test("Ksuid class options and crockford encoding with shuffled alphabet", () => {
  const customAlphabet = "FxnXM1kBN6cuhsAvjW3Co7l2RePyY8DwaU04Tzt9fHQrqSVKdpimLGIJOgb5ZE";
  const client = new Ksuid({
    alphabet: customAlphabet,
    encoding: "crockford",
  });

  const str = client.toString();
  expect(str.length).toBeGreaterThan(0);

  const parsed = Ksuid.parse(str, {
    alphabet: customAlphabet,
    encoding: "crockford",
  });

  expect(parsed.toBase62()).toBe(client.toBase62());
});
