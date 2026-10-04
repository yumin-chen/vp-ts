import { expect, test } from "vite-plus/test";
import { createSecretKey } from "../index.js";

test("KeyObject secret key creation and export", () => {
  const secret = Buffer.from("supersecretkey");
  const keyObj = createSecretKey(secret);
  expect(keyObj.keyType).toBe("secret");
  expect(keyObj.export().toString()).toBe("supersecretkey");
});
