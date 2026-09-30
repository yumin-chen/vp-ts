import { expect, test } from "vite-plus/test";
import {
  KsuidClient,
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

test("KsuidClient class options and crockford encoding", () => {
  const customAlphabet = "ZYXWVUTSRQPONMKJHGFEDCBA98765432";
  const client = new KsuidClient({
    encoding: "crockford",
    alphabet: customAlphabet,
  });

  const str = client.toString();
  expect(str.length).toBeGreaterThan(0);

  const parsed = KsuidClient.parse(str, {
    encoding: "crockford",
    alphabet: customAlphabet,
  });

  expect(parsed.toBase62()).toBe(client.toBase62());
});
