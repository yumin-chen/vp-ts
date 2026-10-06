import assert from "node:assert/strict";
import test from "node:test";
import pkg from "../index.js";

const { Tls, TlsProvider } = pkg;

void test("TLS provider class", () => {
  const tls = new Tls(TlsProvider.Ring);
  assert.equal(tls.getProviderName(), "ring");
  assert.equal(tls.isSupported(), true);

  tls.setAlpnProtocols(["h2", "http/1.1"]);
});
