import { describe, expect, test } from "vite-plus/test";
import { parseUrl } from "./main.ts";

describe("ObjectStore E2E Acceptance Tests", () => {
  test("full object store lifecycle", async () => {
    // Parse URL to create store
    const parsed = parseUrl("memory://e2e/test_bucket");
    const store = parsed.store;

    // 1. Write objects
    await store.put("e2e/file1.txt", Buffer.from("e2e content 1"));
    await store.put("e2e/file2.txt", Buffer.from("e2e content 2"));
    await store.put("e2e/sub/file3.txt", Buffer.from("e2e content 3"));

    // 2. List objects with delimiter
    const listDelim = await store.listWithDelimiter("e2e");
    expect(listDelim.objects.length).toBe(2);
    expect(listDelim.commonPrefixes).toContain("e2e/sub");

    // 3. Range read
    const range = await store.getRange("e2e/file1.txt", 0, 3);
    expect(range.toString()).toBe("e2e");

    // 4. Copy & Rename
    await store.copy("e2e/file1.txt", "e2e/file1_copy.txt");
    await store.rename("e2e/file1_copy.txt", "e2e/file1_renamed.txt");

    const renamed = await store.get("e2e/file1_renamed.txt");
    expect(renamed.bytes.toString()).toBe("e2e content 1");

    // 5. Bulk Delete
    await store.deleteStream(["e2e/file1.txt", "e2e/file2.txt", "e2e/file1_renamed.txt"]);

    const remaining = await store.list("e2e");
    expect(remaining.length).toBe(1);
    expect(remaining[0]?.location).toBe("e2e/sub/file3.txt");
  });
});
