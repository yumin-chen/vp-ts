import { expect, test } from "vite-plus/test";
import { Hmac, TLS, createHash, pbkdf2Sync, randomBytes } from "./main.ts";

test("src/main exports crypto bindings", () => {
  expect(typeof createHash).toBe("function");
  expect(typeof pbkdf2Sync).toBe("function");
  expect(typeof randomBytes).toBe("function");

  const hmac = new Hmac("sha256", "key");
  hmac.update("data");
  expect(hmac.digest("hex")).toBeDefined();

  const tls = new TLS();
  expect(tls.providerName).toBe("ring");
});
