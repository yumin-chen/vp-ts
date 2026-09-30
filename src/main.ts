import { customAlphabetNative, formatNative, nanoidNative, URL_ALPHABET } from "./native.js";

export const urlAlphabet = URL_ALPHABET;

function validateAlphabet(alphabet: string): void {
  if (alphabet.length === 0 || alphabet.length > 256) {
    throw new Error("Alphabet must contain from 1 to 256 symbols.");
  }
}

export function nanoid(size?: number): string {
  return nanoidNative(size);
}

export function customAlphabet(alphabet: string, defaultSize = 21) {
  validateAlphabet(alphabet);
  return (size?: number): string => {
    return customAlphabetNative(alphabet, size ?? defaultSize);
  };
}

export function customRandom(
  alphabet: string,
  defaultSize: number,
  random: (size: number) => Uint8Array | number[],
) {
  validateAlphabet(alphabet);
  const step = Math.ceil(
    (1.6 * (1 << Math.ceil(Math.log2(alphabet.length))) * defaultSize) / alphabet.length,
  );

  return (size?: number): string => {
    const targetSize = size ?? defaultSize;
    let id = "";
    while (id.length < targetSize) {
      const bytes = random(step);
      const buf = Buffer.from(bytes);
      const remaining = targetSize - id.length;
      const formatted = formatNative(alphabet, remaining, buf);
      id += formatted;
    }
    return id;
  };
}
