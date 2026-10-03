import { describe, expect, it } from "vite-plus/test";
import { TLS } from "../index.js";

describe("TLS", () => {
  it("should create default TLS instance with ring provider", () => {
    const tls = new TLS();
    expect(tls.getProvider()).toBe("ring");
    expect(tls.isSupported()).toBe(true);
    expect(tls.connect("example.com", 443)).toContain("ring");
  });

  it("should support openssl, boringssl, mbedtls providers", () => {
    const openssl = new TLS("openssl");
    expect(openssl.getProvider()).toBe("openssl");

    const btls = new TLS("boringssl");
    expect(btls.getProvider()).toBe("boringssl");

    const mbedtls = new TLS("mbedtls");
    expect(mbedtls.getProvider()).toBe("mbedtls");
  });
});
