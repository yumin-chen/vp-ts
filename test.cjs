const assert = require("node:assert/strict");
const test = require("node:test");
const {
  createHmac,
  Hmac,
  pbkdf2Sync,
  pbkdf2,
  Pbkdf2,
  Tls,
  createTls,
  getDefaultProviderName,
  getSupportedProviders,
  Hasher,
  createHash,
  hash,
  getHashes,
} = require("./index.js");

test("HMAC - sha256 hex", () => {
  const key = Buffer.from("a secret");
  const hmac = createHmac("sha256", key);
  hmac.update(Buffer.from("hello world"));
  const digest = hmac.digest("hex");
  assert.equal(digest, "322d4e7e52c59af88c8290fdbf52579a32d1e30f1d8b0a34808771e95cc88ab3");
});

test("HMAC - buffer digest and throw after digest", () => {
  const key = Buffer.from("secret");
  const hmac = new Hmac("sha256", key);
  hmac.update(Buffer.from("test data"));
  const buf = hmac.digest();
  assert.ok(Buffer.isBuffer(buf));
  assert.equal(buf.length, 32);

  assert.throws(() => hmac.update(Buffer.from("more")));
  assert.throws(() => hmac.digest("hex"));
});

test("PBKDF2 - sync and async derive", async () => {
  const password = Buffer.from("password");
  const salt = Buffer.from("salt");
  const syncDerived = pbkdf2Sync(password, salt, 1000, 32, "sha256");
  assert.equal(syncDerived.length, 32);
  assert.equal(syncDerived.toString("hex"), "632c2812e46d4604102ba7618e9d6d7d2f8128f6266b4a03264d2a0460b7dcb3");

  const asyncDerived = await pbkdf2(password, salt, 1000, 32, "sha256");
  assert.equal(asyncDerived.toString("hex"), syncDerived.toString("hex"));

  const instance = new Pbkdf2(500, 16, "sha256");
  const instSync = instance.deriveSync(password, salt);
  const instAsync = await instance.derive(password, salt);
  assert.equal(instSync.length, 16);
  assert.equal(instAsync.toString("hex"), instSync.toString("hex"));
});

test("TLS - provider selection", () => {
  const tlsDefault = new Tls();
  assert.equal(tlsDefault.getProviderName(), "ring");
  assert.equal(tlsDefault.isSupported(), true);

  const tlsOpenssl = createTls("openssl");
  assert.equal(tlsOpenssl.getProviderName(), "openssl");

  const tlsBtls = new Tls("btls");
  assert.equal(tlsBtls.getProviderName(), "btls");

  const tlsMbed = new Tls("mbedtls");
  assert.equal(tlsMbed.getProviderName(), "mbedtls");

  assert.equal(getDefaultProviderName(), "ring");
  const providers = getSupportedProviders();
  assert.ok(providers.includes("ring"));
  assert.ok(providers.includes("openssl"));
  assert.ok(providers.includes("btls"));
  assert.ok(providers.includes("mbedtls"));

  assert.throws(() => new Tls("invalid_provider"));
});

test("Hasher - createHash, hash, getHashes", () => {
  const hasher = createHash("sha256");
  hasher.update(Buffer.from("hello world"));
  const digest = hasher.digest("hex");
  assert.equal(digest, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");

  const oneShotHex = hash("sha256", Buffer.from("hello world"), "hex");
  assert.equal(oneShotHex, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");

  const hashes = getHashes();
  assert.ok(hashes.includes("sha256"));
  assert.ok(hashes.includes("sha512"));

  assert.throws(() => new Hasher("invalid_algo"));
});
