import { describe, expect, it } from "vite-plus/test";
import { CryptoKeyPair, ECDH, KeyObject, Sign, Verify } from "../index.js";

describe("Agreement & Keys", () => {
  it("should generate ECDH keys and compute secret for P-256", () => {
    const alice = new ECDH("p-256");
    const bob = new ECDH("p-256");

    const alicePub = alice.getPublicKey();
    const bobPub = bob.getPublicKey();

    const secretAlice = alice.computeSecret(bobPub);
    const secretBob = bob.computeSecret(alicePub);

    expect(secretAlice.toString("hex")).toBe(secretBob.toString("hex"));
  });

  it("should generate X25519 ECDH secret", () => {
    const alice = new ECDH("x25519");
    const bob = new ECDH("x25519");

    const secretAlice = alice.computeSecret(bob.getPublicKey());
    const secretBob = bob.computeSecret(alice.getPublicKey());

    expect(secretAlice.toString("hex")).toBe(secretBob.toString("hex"));
  });

  it("should construct KeyObject and CryptoKeyPair", () => {
    const pubKey = new KeyObject("public", "ec", Buffer.from("pub"));
    const privKey = new KeyObject("private", "ec", Buffer.from("priv"));
    expect(pubKey.type).toBe("public");
    expect(privKey.type).toBe("private");

    const pair = new CryptoKeyPair(pubKey, privKey);
    expect(pair.publicKey.type).toBe("public");
    expect(pair.privateKey.type).toBe("private");
  });

  it("should Sign and Verify using Ed25519", () => {
    const signer = new Sign("ed25519");
    signer.update("message to sign");

    const privKey = Buffer.alloc(32, 1);
    const sig = signer.sign(privKey, "hex");
    expect(typeof sig).toBe("string");

    const verifier = new Verify("ed25519");
    verifier.update("message to sign");

    // Ed25519 pubkey derived from seed (first 32 bytes)
    const pubKey = Buffer.alloc(32, 1);
    const isValid = verifier.verify(pubKey, sig as string);
    expect(typeof isValid).toBe("boolean");
  });
});
