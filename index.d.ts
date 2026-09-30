export declare const urlAlphabet: string;

export declare function nanoid(size?: number): string;

export declare function customAlphabet(
  alphabet: string,
  defaultSize?: number,
): (size?: number) => string;

export declare function customRandom(
  alphabet: string,
  defaultSize: number,
  random: (size: number) => Uint8Array | number[],
): (size?: number) => string;
