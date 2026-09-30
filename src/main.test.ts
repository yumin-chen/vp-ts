import { expect, test } from "vite-plus/test";
import {
  BoringSshCodec,
  decodeBase64,
  decrypt,
  digest,
  encodeBase64,
  encrypt,
  hmac,
  init,
  randomBytes,
  sha256,
  sha512,
  version,
} from "./main.ts";

test("init and version", () => {
  init();
  const v = version();
  expect(typeof v).toBe("string");
  expect(v.length).toBeGreaterThan(0);
});

test("encodeBase64 and decodeBase64", () => {
  const input = Buffer.from("Hello BoringSSL!");
  const encoded = encodeBase64(input);
  expect(typeof encoded).toBe("string");
  const decoded = decodeBase64(encoded);
  expect(decoded.toString("utf-8")).toBe("Hello BoringSSL!");
});

test("sha256 and sha512 and digest", () => {
  const input = Buffer.from("Test Hash Message");
  const hash1 = sha256(input);
  expect(hash1.length).toBe(32);

  const hash2 = digest("sha256", input);
  expect(hash1.toString("hex")).toBe(hash2.toString("hex"));

  const hash3 = sha512(input);
  expect(hash3.length).toBe(64);
});

test("encrypt and decrypt using aes-128-cbc", () => {
  const key = Buffer.alloc(16, 1);
  const iv = Buffer.alloc(16, 2);
  const plainText = Buffer.from("Secret Payload 12345678");

  const encrypted = encrypt("aes-128-cbc", key, iv, plainText);
  expect(encrypted).not.toEqual(plainText);

  const decrypted = decrypt("aes-128-cbc", key, iv, encrypted);
  expect(decrypted.toString("utf-8")).toBe("Secret Payload 12345678");
});

test("hmac and randomBytes", () => {
  const key = Buffer.from("secret-key");
  const message = Buffer.from("data message");
  const mac = hmac("sha256", key, message);
  expect(mac.length).toBe(32);

  const rnd = randomBytes(16);
  expect(rnd.length).toBe(16);
});

test("BoringSshCodec class", () => {
  const codec = new BoringSshCodec();
  const data = Buffer.from("Codec Test Data");

  const encoded = codec.encode(data);
  const decoded = codec.decode(encoded);
  expect(decoded.toString("utf-8")).toBe("Codec Test Data");

  const hash = codec.hash("sha256", data);
  expect(hash.length).toBe(32);

  const rnd = codec.randomBytes(24);
  expect(rnd.length).toBe(24);
});
