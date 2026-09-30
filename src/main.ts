import { CrockfordBase32, Ksuid as NativeKsuid, KsuidMs as NativeKsuidMs } from "../index.js";

export { CrockfordBase32 };

export const Ksuid = NativeKsuid as typeof NativeKsuid & {
  PAYLOAD_BYTES: number;
  BYTE_SIZE: number;
};
Ksuid.PAYLOAD_BYTES = 16;
Ksuid.BYTE_SIZE = 20;

export const KsuidMs = NativeKsuidMs as typeof NativeKsuidMs & {
  PAYLOAD_BYTES: number;
  BYTE_SIZE: number;
};
KsuidMs.PAYLOAD_BYTES = 12;
KsuidMs.BYTE_SIZE = 20;

export function generateKsuid(payload?: Uint8Array): string {
  return Ksuid.now(payload).toBase62();
}

export function generateKsuidMs(payload?: Uint8Array): string {
  return KsuidMs.now(payload).toBase62();
}

export default Ksuid;
