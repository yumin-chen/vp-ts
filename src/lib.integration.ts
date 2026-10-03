import { describe, expect, test } from "vite-plus/test";
import { ObjectStore } from "./main.ts";

describe("ObjectStore Integration Tests", () => {
  test("cross-store workflow between memory and local filesystem", async () => {
    const memStore = ObjectStore.inMemory();
    const localStore = ObjectStore.local("/tmp/object_store_test");

    // Put object in memory store
    const payload = Buffer.from("Integration Payload Data");
    await memStore.put("transfer/file1.bin", payload);

    // Fetch from memory store
    const fetched = await memStore.get("transfer/file1.bin");
    expect(fetched.bytes.toString()).toBe("Integration Payload Data");

    // Copy to local store
    await localStore.put("transfer/file1.bin", fetched.bytes);

    // Verify in local store
    const localGet = await localStore.get("transfer/file1.bin");
    expect(localGet.bytes.toString()).toBe("Integration Payload Data");

    // Cleanup local store
    await localStore.delete("transfer/file1.bin");
  });
});
