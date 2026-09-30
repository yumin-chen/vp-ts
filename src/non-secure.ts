import { urlAlphabet } from "./main.js";

function validateAlphabet(alphabet: string): void {
  if (alphabet.length === 0 || alphabet.length > 256) {
    throw new Error("Alphabet must contain from 1 to 256 symbols.");
  }
}

export function customAlphabet(alphabet: string, defaultSize = 21) {
  validateAlphabet(alphabet);
  return (size?: number): string => {
    const targetSize = size ?? defaultSize;
    let id = "";
    while (id.length < targetSize) {
      const bytes = new Uint8Array(targetSize);
      for (let i = 0; i < targetSize; i++) {
        bytes[i] = Math.floor(Math.random() * 256);
      }
      for (let i = 0; i < bytes.length; i++) {
        const byte = bytes[i];
        if (byte < alphabet.length) {
          id += alphabet[byte];
          if (id.length === targetSize) {
            break;
          }
        }
      }
    }
    return id;
  };
}

export function nanoid(size = 21): string {
  return customAlphabet(urlAlphabet, size)();
}
