import { expect, test } from "vite-plus/test";
import {
  createDiffieHellman,
  createEcdh,
  diffieHellman,
  getDiffieHellman,
} from "../artifacts/index.js";

test("ECDH generateKeys and computeSecret", () => {
  const alice = createEcdh("prime256v1");
  const bob = createEcdh("prime256v1");

  alice.generateKeys();
  bob.generateKeys();

  const pubA = alice.getPublicKey("buffer");
  const pubB = bob.getPublicKey("buffer");

  const secretA = alice.computeSecret(pubB);
  const secretB = bob.computeSecret(pubA);

  expect(secretA.toString("hex")).toBe(secretB.toString("hex"));
});

test("createDiffieHellman and getDiffieHellman", () => {
  const dh = createDiffieHellman("modp14");
  const keys = dh.generateKeys();
  expect(Buffer.isBuffer(keys)).toBe(true);

  const group = getDiffieHellman("modp14");
  const groupKeys = group.generateKeys();
  expect(Buffer.isBuffer(groupKeys)).toBe(true);

  const secret = diffieHellman(Buffer.from([1, 2, 3]), Buffer.from([4, 5, 6]));
  expect(Buffer.isBuffer(secret)).toBe(true);
});
