import { CrockfordBase32, Ksuid as NativeKsuid, KsuidMs as NativeKsuidMs } from "../index.js";

export { CrockfordBase32 };

export type EncodingType = "base62" | "crockfordBase32" | "base32";

let globalDefaultEncoding: EncodingType = "base62";
let globalCrockfordEncoder: CrockfordBase32 | undefined;

export function setDefaultEncoding(encoding: EncodingType, encoder?: CrockfordBase32): void {
  globalDefaultEncoding = encoding;
  globalCrockfordEncoder = encoder;
}

export function getDefaultEncoding(): { encoding: EncodingType; encoder?: CrockfordBase32 } {
  return { encoding: globalDefaultEncoding, encoder: globalCrockfordEncoder };
}

// Override toString on NativeKsuid prototype to support default encoding
NativeKsuid.prototype.toString = function (
  this: NativeKsuid,
  encoding?: string,
  encoder?: CrockfordBase32,
): string {
  const enc = encoding || globalDefaultEncoding;
  if (enc === "crockfordBase32" || enc === "base32") {
    return this.toCrockfordBase32(encoder || globalCrockfordEncoder);
  }
  return this.toBase62();
};

NativeKsuidMs.prototype.toString = function (
  this: NativeKsuidMs,
  encoding?: string,
  encoder?: CrockfordBase32,
): string {
  const enc = encoding || globalDefaultEncoding;
  if (enc === "crockfordBase32" || enc === "base32") {
    return this.toCrockfordBase32(encoder || globalCrockfordEncoder);
  }
  return this.toBase62();
};

export const Ksuid = NativeKsuid as typeof NativeKsuid & {
  PAYLOAD_BYTES: number;
  BYTE_SIZE: number;
  setDefaultEncoding: typeof setDefaultEncoding;
};
Ksuid.PAYLOAD_BYTES = 16;
Ksuid.BYTE_SIZE = 20;
Ksuid.setDefaultEncoding = setDefaultEncoding;

export const KsuidMs = NativeKsuidMs as typeof NativeKsuidMs & {
  PAYLOAD_BYTES: number;
  BYTE_SIZE: number;
  setDefaultEncoding: typeof setDefaultEncoding;
};
KsuidMs.PAYLOAD_BYTES = 12;
KsuidMs.BYTE_SIZE = 20;
KsuidMs.setDefaultEncoding = setDefaultEncoding;

export function generateKsuid(payload?: Uint8Array): string {
  return Ksuid.now(payload).toString();
}

export function generateKsuidMs(payload?: Uint8Array): string {
  return KsuidMs.now(payload).toString();
}

export default Ksuid;
