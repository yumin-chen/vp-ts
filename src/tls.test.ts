import { expect, test } from "vite-plus/test";
import { Tls } from "../index.js";

test("Tls provider default and options", () => {
  const tlsDefault = new Tls();
  expect(tlsDefault.provider).toBe("ring");
  expect(tlsDefault.ciphersuites.length).toBeGreaterThan(0);

  const tlsOpenssl = new Tls("openssl");
  expect(tlsOpenssl.provider).toBe("openssl");
});
