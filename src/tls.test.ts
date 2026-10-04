import { expect, test } from "vite-plus/test";
import { createTls, Tls } from "../index.js";

test("TLS provider default and custom provider", () => {
  const tlsRing = new Tls();
  expect(tlsRing.getProvider()).toBe("ring");

  const tlsCustom = createTls("rustls");
  expect(tlsCustom.getProvider()).toBe("rustls");
});
