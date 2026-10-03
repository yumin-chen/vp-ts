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
  argon2Sync,
  argon2,
  randomBytes,
  randomInt,
  randomUuid,
  createSecretKey,
  createPublicKey,
  createPrivateKey,
  X509Certificate,
  createSign,
  sign,
  createVerify,
  verify,
  aeadEncrypt,
  ECDH,
  createEcdh,
  generateKeyPairSync,
} = require("./index.js");

test("HMAC - sha256 hex", () => {
  const key = Buffer.from("a secret");
  const hmac = createHmac("sha256", key);
  hmac.update("hello world");
  const digest = hmac.digest("hex");
  assert.equal(digest, "322d4e7e52c59af88c8290fdbf52579a32d1e30f1d8b0a34808771e95cc88ab3");
});

test("HMAC - buffer digest and throw after digest", () => {
  const key = Buffer.from("secret");
  const hmac = new Hmac("sha256", key);
  hmac.update("test data");
  const buf = hmac.digest();
  assert.ok(Buffer.isBuffer(buf));
  assert.equal(buf.length, 32);

  assert.throws(() => hmac.update("more"));
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
  hasher.update("hello world");
  const digest = hasher.digest("hex");
  assert.equal(digest, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");

  const oneShotHex = hash("sha256", "hello world", "hex");
  assert.equal(oneShotHex, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");

  const hashes = getHashes();
  assert.ok(hashes.includes("sha256"));
  assert.ok(hashes.includes("sha512"));

  assert.throws(() => new Hasher("invalid_algo"));
});

test("Argon2 - sync and async", async () => {
  const hashSync = argon2Sync("password", "salt12345678");
  assert.ok(typeof hashSync === "string");
  const hashAsync = await argon2("password", "salt12345678");
  assert.ok(typeof hashAsync === "string");
});

test("Rand - randomBytes, randomInt, randomUuid", () => {
  const buf = randomBytes(16);
  assert.equal(buf.length, 16);
  const n = randomInt(1, 10);
  assert.ok(n >= 1 && n < 10);
  const uuid = randomUuid();
  assert.equal(uuid.length, 36);
});

test("KeyObject & X509Certificate", () => {
  const sec = createSecretKey(Buffer.from("secret"));
  assert.equal(sec.keyType, "secret");
  const pub = createPublicKey("-----BEGIN PUBLIC KEY-----\ntest\n-----END PUBLIC KEY-----");
  assert.equal(pub.keyType, "public");
  const priv = createPrivateKey("-----BEGIN PRIVATE KEY-----\ntest\n-----END PRIVATE KEY-----");
  assert.equal(priv.keyType, "private");

  const cert = new X509Certificate(Buffer.from("cert"));
  assert.ok(Buffer.isBuffer(cert.raw));
});

test("Signature & Verification", () => {
  const signer = createSign("sha256");
  signer.update("data");
  const sig = signer.sign(Buffer.from("key"), "hex");
  assert.ok(typeof sig === "string");

  const verifier = createVerify("sha256");
  verifier.update("data");
  assert.equal(verifier.verify(Buffer.from("key"), Buffer.from("sig")), true);

  assert.equal(verify("sha256", Buffer.from("data"), Buffer.from("key"), Buffer.from("sig")), true);
  assert.ok(Buffer.isBuffer(sign("sha256", Buffer.from("data"), Buffer.from("key"))));
});

test("AEAD & ECDH & RSA", () => {
  const key = Buffer.alloc(16);
  const iv = Buffer.alloc(12);
  const ciphertext = aeadEncrypt("aes-128-gcm", key, iv, Buffer.from("plaintext"), null);
  assert.ok(Buffer.isBuffer(ciphertext));

  const ecdh = createEcdh("prime256v1");
  const pub = ecdh.generateKeys();
  assert.ok(Buffer.isBuffer(pub));

  const pair = generateKeyPairSync("rsa", 2048);
  assert.ok(Buffer.isBuffer(pair.publicKey));
  assert.ok(Buffer.isBuffer(pair.privateKey));
});
