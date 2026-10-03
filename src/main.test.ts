import { describe, expect, it } from "vite-plus/test";
import { getHashes } from "../index.js";

describe("main", () => {
  it("should return hashes", () => {
    expect(getHashes()).toContain("sha256");
  });
});
