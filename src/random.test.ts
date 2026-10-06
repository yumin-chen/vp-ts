import { describe, expect, it } from "vite-plus/test";
import {
  randomBytes,
  randomFill,
  randomFillSync,
  randomInt,
  randomUUID,
  randomUUIDv7,
} from "./main.js";

describe("random utilities", () => {
  it("generates sync randomBytes", () => {
    const buf = randomBytes(256) as Buffer;
    expect(Buffer.isBuffer(buf)).toBe(true);
    expect(buf.length).toBe(256);
  });

  it("generates async randomBytes with callback", async () => {
    const promise = new Promise<Buffer>((resolve, reject) => {
      randomBytes(128, (err, buf) => (err ? reject(err) : resolve(buf)));
    });
    const buf = await promise;
    expect(Buffer.isBuffer(buf)).toBe(true);
    expect(buf.length).toBe(128);
  });

  it("fills buffer with randomFillSync", () => {
    const buf = Buffer.alloc(10);
    const filled = randomFillSync(buf, 0, 10);
    expect(filled).toBe(buf);
  });

  it("fills buffer with async randomFill", async () => {
    const buf = Buffer.alloc(10);
    const promise = new Promise<Buffer>((resolve, reject) => {
      randomFill(buf, (err: any, filled: Buffer) => (err ? reject(err) : resolve(filled)));
    });
    const filled = await promise;
    expect(filled).toBe(buf);
  });

  it("generates randomInt within range", () => {
    const n = randomInt(1, 10) as number;
    expect(n).toBeGreaterThanOrEqual(1);
    expect(n).toBeLessThan(10);
  });

  it("generates randomUUID (v4)", () => {
    const uuid = randomUUID();
    expect(typeof uuid).toBe("string");
    expect(uuid).toMatch(/^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i);
  });

  it("generates randomUUIDv7", () => {
    const uuid = randomUUIDv7();
    expect(typeof uuid).toBe("string");
    expect(uuid).toMatch(/^[0-9a-f]{8}-[0-9a-f]{4}-7[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i);
  });
});
