import assert from "node:assert/strict";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";

import { BufReader, BufWriter, ObjectStore, parseUrl } from "../index.js";

test("ObjectStore integration - memory & local store", async () => {
  const store = ObjectStore.memory();

  await store.put("hello.txt", Buffer.from("world"));
  const data = await store.get("hello.txt");
  assert.equal(data.toString("utf8"), "world");

  const meta = await store.head("hello.txt");
  assert.equal(meta.size, 5);

  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "obj-store-root-test-"));
  try {
    const localStore = ObjectStore.local(tmpDir);
    await localStore.put("file.txt", Buffer.from("local content"));
    const localData = await localStore.get("file.txt");
    assert.equal(localData.toString("utf8"), "local content");
  } finally {
    fs.rmSync(tmpDir, { recursive: true, force: true });
  }
});

test("ObjectStore integration - parseUrl & buffered IO", async () => {
  const parsed = parseUrl("memory://");
  assert.ok(parsed.store);

  const writer = new BufWriter(parsed.store, "stream.txt");
  await writer.write(Buffer.from("buffered "));
  await writer.write(Buffer.from("data"));
  await writer.finish();

  const reader = await BufReader.open(parsed.store, "stream.txt");
  const readData = await reader.read(13);
  assert.equal(readData.toString("utf8"), "buffered data");
});
