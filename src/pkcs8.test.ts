import { expect, test } from "vite-plus/test";
import { parsePkcs8, exportPkcs8 } from "../index.js";

test("parsePkcs8 returns info", () => {
  const info = parsePkcs8(Buffer.from("dummy key data"));
  expect(info.algorithmOid).toBeDefined();
  expect(typeof info.isEncrypted).toBe("boolean");
});

test("exportPkcs8 exports PEM formatted string", () => {
  const pem = exportPkcs8(Buffer.from("raw key"));
  expect(pem).toContain("BEGIN PRIVATE KEY");
  expect(pem).toContain("END PRIVATE KEY");
});
