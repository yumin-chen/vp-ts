import { expect, test } from "vite-plus/test";
import { Tls, createTls } from "../dist/index.js";

test("TLS default provider is ring", () => {
  const tls = new Tls();
  expect(tls.providerName()).toBe("ring");
  expect(tls.getCryptoProvider()).toBe("rustls::crypto::ring");
  expect(tls.isSupported()).toBe(true);
});

test("TLS openssl provider", () => {
  const tls = new Tls("openssl");
  expect(tls.providerName()).toBe("openssl");
  expect(tls.getCryptoProvider()).toBe("rustls-openssl");
  expect(tls.isSupported()).toBe(true);
});

test("TLS boringssl provider", () => {
  const tls = createTls("boringssl");
  expect(tls.providerName()).toBe("boringssl");
  expect(tls.getCryptoProvider()).toBe("boring-rustls-provider");
  expect(tls.isSupported()).toBe(true);
});

test("TLS mbedtls provider", () => {
  const tls = createTls("mbedtls");
  expect(tls.providerName()).toBe("mbedtls");
  expect(tls.getCryptoProvider()).toBe("rustls-mbedtls-provider");
  expect(tls.isSupported()).toBe(true);
});
