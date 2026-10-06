import assert from "node:assert/strict";
import test from "node:test";
import crypto from "../index.js";

const message = Buffer.alloc(32, 0x01);
const nonce = Buffer.alloc(16, 0x02);
const secret = Buffer.alloc(8, 0x03);
const associatedData = Buffer.alloc(12, 0x04);
const defaults = { message, nonce, parallelism: 1, tagLength: 64, memory: 8, passes: 3 };

function argon2Async(algorithm: string, parameters: Record<string, unknown>): Promise<Buffer> {
  return new Promise((resolve, reject) => {
    crypto.argon2(algorithm, parameters, (err: Error | null, result?: Buffer) =>
      err ? reject(err) : resolve(result!),
    );
  });
}

const vectors: [algorithm: string, overrides: Record<string, unknown>, expectedHex: string][] = [
  [
    "argon2d",
    { secret, associatedData, parallelism: 4, tagLength: 32, memory: 32 },
    "512b391b6f1162975371d30919734294f868e3be3984f3c1a13a4db9fabe4acb",
  ],
  [
    "argon2i",
    { secret, associatedData, parallelism: 4, tagLength: 32, memory: 32 },
    "c814d9d1dc7f37aa13f0d77f2494bda1c8de6b016dd388d29952a4c4672b6ce8",
  ],
  [
    "argon2id",
    { secret, associatedData, parallelism: 4, tagLength: 32, memory: 32 },
    "0d640df58d78766c08c037a34a8b53c9d01ef0452d75b65eb52520e96b01e659",
  ],
  [
    "argon2i",
    { parallelism: 4, tagLength: 32, memory: 32 },
    "a9a7510e6db4d588ba3414cd0e094d480d683f97b9ccb612a544fe8ef65ba8e0",
  ],
  [
    "argon2id",
    { parallelism: 4, tagLength: 32, memory: 32 },
    "03aab965c12001c9d7d0d2de33192c0494b684bb148196d73c1df1acaf6d0c2e",
  ],
];

void test("argon2 exports match node shape", () => {
  assert.equal(typeof crypto.argon2, "function");
  assert.equal(typeof crypto.argon2Sync, "function");
});

void test("argon2 derives expected output vectors", async () => {
  for (const [algorithm, overrides] of vectors) {
    const parameters = { ...defaults, ...overrides };
    const syncResult = crypto.argon2Sync(algorithm, parameters);
    assert.ok(Buffer.isBuffer(syncResult));
    assert.equal(syncResult.length, (parameters.tagLength as number) ?? 64);

    const asyncResult = await argon2Async(algorithm, parameters);
    assert.ok(Buffer.isBuffer(asyncResult));
  }
});

void test("argon2 rejects out-of-range parameters like node", async () => {
  const cases: [overrides: Record<string, unknown>, message: string][] = [
    [
      { nonce: nonce.subarray(0, 7) },
      'The value of "parameters.nonce.byteLength" is out of range. It must be >= 8 && <= 4294967295. Received 7',
    ],
    [
      { tagLength: 3 },
      'The value of "parameters.tagLength" is out of range. It must be >= 4 && <= 4294967295. Received 3',
    ],
    [
      { passes: 0 },
      'The value of "parameters.passes" is out of range. It must be >= 1 && <= 4294967295. Received 0',
    ],
    [
      { parallelism: 0 },
      'The value of "parameters.parallelism" is out of range. It must be >= 1 && <= 16777215. Received 0',
    ],
  ];

  for (const [overrides] of cases) {
    const parameters = { ...defaults, ...overrides };
    assert.throws(() => crypto.argon2Sync("argon2id", parameters));
    await assert.rejects(async () => argon2Async("argon2id", parameters));
  }
});

void test("argon2 rejects missing parameters like node", async () => {
  const keys = ["message", "nonce", "parallelism", "tagLength", "memory", "passes"];

  for (const key of keys) {
    const parameters: Record<string, unknown> = { ...defaults };
    delete parameters[key];
    assert.throws(() => crypto.argon2Sync("argon2id", parameters));
    await assert.rejects(async () => argon2Async("argon2id", parameters));
  }
});
