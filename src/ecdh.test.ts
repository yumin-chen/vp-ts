import { expect, test } from "vite-plus/test";
import { createECDH, createDiffieHellman, createDiffieHellmanGroup } from "../index.js";

test("createECDH generates keys and computes shared secret", () => {
  const alice = createECDH("prime256v1");
  const bob = createECDH("prime256v1");

  alice.generateKeys();
  bob.generateKeys();

  const aliceSecret = alice.computeSecret(bob.getPublicKey());
  const bobSecret = bob.computeSecret(alice.getPublicKey());

  expect(aliceSecret.toString("hex")).toBe(bobSecret.toString("hex"));
});

test("createDiffieHellman and createDiffieHellmanGroup return ECDH instances", () => {
  const dh1 = createDiffieHellman(2048);
  expect(dh1).toBeDefined();
  const dh2 = createDiffieHellmanGroup("modp14");
  expect(dh2).toBeDefined();
});
