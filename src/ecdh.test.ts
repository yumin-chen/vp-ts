import { expect, test } from "vite-plus/test";
import {
  ECDH,
  createECDH,
  DiffieHellman,
  createDiffieHellman,
  createDiffieHellmanGroup,
} from "../dist/index.js";

test("ECDH key generation and computeSecret", () => {
  const alice = createECDH("prime256v1");
  const bob = new ECDH("prime256v1");

  const alicePub = alice.generateKeys();
  const bobPub = bob.generateKeys();

  expect(Buffer.isBuffer(alicePub)).toBe(true);
  expect(Buffer.isBuffer(bobPub)).toBe(true);

  const secretA = alice.computeSecret(bobPub);
  expect(Buffer.isBuffer(secretA)).toBe(true);
});

test("DiffieHellman key generation", () => {
  const dh = createDiffieHellman(1024);
  const pub = dh.generateKeys();
  expect(Buffer.isBuffer(pub)).toBe(true);
  expect(Buffer.isBuffer(dh.getPrime())).toBe(true);

  const group = createDiffieHellmanGroup("modp14");
  expect(group.getPrime().length).toBe(256);
});
