export interface SqidsOptions {
  alphabet?: string;
  minLength?: number;
  blocklist?: Set<string> | Iterable<string>;
}

export const defaultOptions: {
  alphabet: string;
  minLength: number;
  blocklist: Set<string>;
};

export default class Sqids {
  constructor(options?: SqidsOptions);
  encode(numbers: number[]): string;
  decode(id: string): number[];
}

export { Sqids };
