import { expect, test } from "vite-plus/test";
import { TLS } from "../index.js";

test("TLS default provider is ring", () => {
  const tls = new TLS();
  expect(tls.provider).toBe("ring");
  expect(tls.getProvider()).toBe("ring");
});

test("TLS supports specifying providers", () => {
  const opensslTls = new TLS("openssl");
  expect(opensslTls.provider).toBe("openssl");

  const boringsslTls = new TLS("btls");
  expect(boringsslTls.provider).toBe("boringssl");

  const mbedTls = new TLS("mbedtls");
  expect(mbedTls.provider).toBe("mbedtls");
});

test("TLS getAvailableProviders and isProviderSupported static methods", () => {
  const providers = TLS.getAvailableProviders();
  expect(providers).toContain("ring");
  expect(providers).toContain("openssl");
  expect(providers).toContain("btls");
  expect(providers).toContain("mbedtls");

  expect(TLS.isProviderSupported("ring")).toBe(true);
  expect(TLS.isProviderSupported("openssl")).toBe(true);
  expect(TLS.isProviderSupported("invalid")).toBe(false);
});

test("TLS createClientConfig succeeds for each provider", () => {
  for (const provider of ["ring", "openssl", "btls", "mbedtls"]) {
    const tls = new TLS(provider);
    const result = tls.createClientConfig();
    expect(result).toContain("ClientConfig created");
  }
});
