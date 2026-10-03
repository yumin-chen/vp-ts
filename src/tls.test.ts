import { expect, test } from "vite-plus/test";
import { TLS } from "../artifacts/index.js";

test("TLS default instance uses ring", () => {
  const tls = new TLS();
  expect(tls.providerName).toBe("ring");
  expect(tls.getProviderName()).toBe("ring");
  expect(tls.isFallback).toBe(false);
});

test("TLS supports openssl provider", () => {
  const tls = new TLS("openssl");
  expect(tls.getProviderName()).toBe("openssl");
  expect(tls.getCipherSuites().length).toBeGreaterThan(0);
});

test("TLS supports btls (boringssl) provider", () => {
  const tls = new TLS("btls");
  expect(tls.getProviderName()).toBe("btls");
  expect(tls.getCipherSuites().length).toBeGreaterThan(0);
});

test("TLS supports mbedtls provider", () => {
  const tls = new TLS("mbedtls");
  expect(tls.getProviderName()).toBe("mbedtls");
  expect(tls.getCipherSuites().length).toBeGreaterThan(0);
});

test("TLS.getAvailableProviders returns all supported providers", () => {
  const providers = TLS.getAvailableProviders();
  expect(providers).toContain("ring");
  expect(providers).toContain("openssl");
  expect(providers).toContain("btls");
  expect(providers).toContain("mbedtls");
});

test("TLS falls back to ring on unknown provider", () => {
  const tls = new TLS("nonexistent");
  expect(tls.getProviderName()).toBe("ring");
  expect(tls.isFallback).toBe(true);
});
