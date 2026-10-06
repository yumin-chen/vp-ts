import { describe, expect, it } from "vite-plus/test";
import { createPrivateKey, createPublicKey, createSecretKey } from "./main.js";

describe("KeyObject API", () => {
  it("creates secret key object", () => {
    const secretKey = createSecretKey("my-secret-key");
    expect(secretKey.type).toBe("secret");
    expect(secretKey.symmetricKeySize).toBe(13);
  });

  it("creates public key object", () => {
    const pubKey = createPublicKey("my-public-key");
    expect(pubKey.type).toBe("public");
    expect(pubKey.asymmetricKeyType).toBe("rsa");
  });

  it("creates private key object", () => {
    const privKey = createPrivateKey("my-private-key");
    expect(privKey.type).toBe("private");
    expect(privKey.asymmetricKeyType).toBe("rsa");
  });

  it("compares key objects for equality", () => {
    const k1 = createSecretKey("key1");
    const k2 = createSecretKey("key1");
    const k3 = createSecretKey("key2");

    expect(k1.equals(k2)).toBe(true);
    expect(k1.equals(k3)).toBe(false);
  });

  it("exports key material in raw or pem format", () => {
    const key = createSecretKey("raw-bytes");
    const exported = key.export();
    expect(Buffer.isBuffer(exported)).toBe(true);
  });
});
