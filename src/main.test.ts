import { describe, expect, it } from "vite-plus/test";
import { customAlphabet, customRandom, nanoid, urlAlphabet } from "./main.ts";
import {
  customAlphabet as nonSecureCustomAlphabet,
  nanoid as nonSecureNanoid,
} from "./non-secure.ts";

describe("nanoid API", () => {
  it("exports urlAlphabet", () => {
    expect(typeof urlAlphabet).toBe("string");
    expect(urlAlphabet.length).toBeGreaterThan(0);
  });

  it("generates default 21 character nanoid", () => {
    const id = nanoid();
    expect(typeof id).toBe("string");
    expect(id.length).toBe(21);
    for (const char of id) {
      expect(urlAlphabet.includes(char)).toBe(true);
    }
  });

  it("supports custom size in nanoid(size)", () => {
    const id = nanoid(10);
    expect(id.length).toBe(10);
  });

  it("customAlphabet works with default and overridden size", () => {
    const generator = customAlphabet("1234567890abcdef", 10);
    const id1 = generator();
    expect(id1.length).toBe(10);
    for (const char of id1) {
      expect("1234567890abcdef".includes(char)).toBe(true);
    }

    const id2 = generator(5);
    expect(id2.length).toBe(5);
  });

  it("customRandom works with custom generator", () => {
    let sequenceIndex = 0;
    const fakeBytes = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    const generator = customRandom("abcdef", 10, (size) => {
      const result = new Uint8Array(size);
      for (let i = 0; i < size; i++) {
        result[i] = fakeBytes[sequenceIndex++ % fakeBytes.length];
      }
      return result;
    });

    const id = generator();
    expect(typeof id).toBe("string");
    expect(id.length).toBe(10);
    for (const char of id) {
      expect("abcdef".includes(char)).toBe(true);
    }
  });

  it("validates alphabet size (1 to 256 symbols)", () => {
    expect(() => customAlphabet("")).toThrow("Alphabet must contain from 1 to 256 symbols");
    expect(() => customRandom("", 10, (s) => new Uint8Array(s))).toThrow(
      "Alphabet must contain from 1 to 256 symbols",
    );

    const oversized = "a".repeat(257);
    expect(() => customAlphabet(oversized)).toThrow("Alphabet must contain from 1 to 256 symbols");
    expect(() => customRandom(oversized, 10, (s) => new Uint8Array(s))).toThrow(
      "Alphabet must contain from 1 to 256 symbols",
    );
  });

  it("non-secure nanoid and customAlphabet work as expected", () => {
    const id = nonSecureNanoid();
    expect(id.length).toBe(21);

    const generator = nonSecureCustomAlphabet("1234567890abcdef", 10);
    const customId = generator(5);
    expect(customId.length).toBe(5);
    for (const char of customId) {
      expect("1234567890abcdef".includes(char)).toBe(true);
    }
  });
});
