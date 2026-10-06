import assert from "node:assert/strict";
import test from "node:test";
import pkg from "../index.js";

const { createEcdh, getCurves } = pkg;

void test("ECDH key exchange prime256v1", () => {
  const alice = createEcdh("prime256v1");
  const bob = createEcdh("prime256v1");

  const alicePub = alice.generateKeys();
  const bobPub = bob.generateKeys();

  const aliceSecret = alice.computeSecret(bobPub);
  const bobSecret = bob.computeSecret(alicePub);

  assert.equal(aliceSecret.toString("hex"), bobSecret.toString("hex"));
});

void test("getCurves", () => {
  const curves = getCurves();
  assert.ok(curves.includes("prime256v1"));
});
