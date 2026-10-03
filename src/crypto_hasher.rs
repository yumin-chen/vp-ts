use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::digest::{
  Algorithm, Context, SHA1_FOR_LEGACY_USE_ONLY, SHA256, SHA384, SHA512, SHA512_256,
};
use ring::hkdf::{KeyType, Salt, HKDF_SHA256, HKDF_SHA384, HKDF_SHA512};
use ring::rand::{SecureRandom, SystemRandom};

use crate::hmac::{decode_input, encode_output, to_hex};

fn get_hash_algorithm(algo: &str) -> napi::Result<&'static Algorithm> {
  match algo.to_lowercase().replace("-", "").as_str() {
    "sha1" => Ok(&SHA1_FOR_LEGACY_USE_ONLY),
    "sha256" => Ok(&SHA256),
    "sha384" => Ok(&SHA384),
    "sha512" => Ok(&SHA512),
    "sha512256" => Ok(&SHA512_256),
    _ => Err(napi::Error::from_reason(format!("Unsupported hash algorithm: {}", algo))),
  }
}

#[napi(js_name = "Hash")]
pub struct Hash {
  ctx: Option<Context>,
}

#[napi]
impl Hash {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> napi::Result<Self> {
    let algo = get_hash_algorithm(&algorithm)?;
    let ctx = Context::new(algo);
    Ok(Hash { ctx: Some(ctx) })
  }

  #[napi(ts_args_type = "data: string | Buffer, encoding?: string")]
  pub fn update(&mut self, data: Either<String, Buffer>, encoding: Option<String>) -> napi::Result<()> {
    if let Some(ctx) = &mut self.ctx {
      let bytes = decode_input(&data, encoding.as_deref());
      ctx.update(&bytes);
      Ok(())
    } else {
      Err(napi::Error::from_reason("Hash digest already called"))
    }
  }

  #[napi(ts_return_type = "string | Buffer")]
  pub fn digest(&mut self, encoding: Option<String>) -> napi::Result<Either<String, Buffer>> {
    if let Some(ctx) = self.ctx.take() {
      let digest = ctx.finish();
      Ok(encode_output(digest.as_ref(), encoding.as_deref()))
    } else {
      Err(napi::Error::from_reason("Hash digest already called"))
    }
  }
}

#[napi]
pub fn create_hash(algorithm: String) -> napi::Result<Hash> {
  Hash::new(algorithm)
}

#[napi(
  ts_args_type = "algorithm: string, data: string | Buffer, outputEncoding?: string",
  ts_return_type = "string | Buffer"
)]
pub fn hash(
  algorithm: String,
  data: Either<String, Buffer>,
  output_encoding: Option<String>,
) -> napi::Result<Either<String, Buffer>> {
  let mut h = Hash::new(algorithm)?;
  h.update(data, None)?;
  h.digest(output_encoding)
}

#[napi]
pub fn random_bytes(size: u32) -> napi::Result<Buffer> {
  let rng = SystemRandom::new();
  let mut buf = vec![0u8; size as usize];
  rng
    .fill(&mut buf)
    .map_err(|_| napi::Error::from_reason("Failed to generate random bytes"))?;
  Ok(Buffer::from(buf))
}

#[napi]
pub fn random_fill_sync(
  mut buffer: Buffer,
  offset: Option<u32>,
  size: Option<u32>,
) -> napi::Result<Buffer> {
  let start = offset.unwrap_or(0) as usize;
  let len = size.unwrap_or((buffer.len() - start) as u32) as usize;
  if start + len > buffer.len() {
    return Err(napi::Error::from_reason("Offset + size exceeds buffer length"));
  }
  let rng = SystemRandom::new();
  rng
    .fill(&mut buffer[start..start + len])
    .map_err(|_| napi::Error::from_reason("Failed to fill random bytes"))?;
  Ok(buffer)
}

#[napi]
pub fn random_uuid() -> napi::Result<String> {
  let rng = SystemRandom::new();
  let mut bytes = [0u8; 16];
  rng
    .fill(&mut bytes)
    .map_err(|_| napi::Error::from_reason("Failed to generate UUID"))?;

  bytes[6] = (bytes[6] & 0x0f) | 0x40; // Version 4
  bytes[8] = (bytes[8] & 0x3f) | 0x80; // Variant 1

  let hex = to_hex(&bytes);
  Ok(format!(
    "{}-{}-{}-{}-{}",
    &hex[0..8],
    &hex[8..12],
    &hex[12..16],
    &hex[16..20],
    &hex[20..32]
  ))
}

#[napi(ts_args_type = "a: Buffer, b: Buffer")]
pub fn timing_safe_equal(a: Buffer, b: Buffer) -> napi::Result<bool> {
  if a.len() != b.len() {
    return Err(napi::Error::from_reason("Input buffers must have the same length"));
  }
  let mut res = 0u8;
  for (x, y) in a.iter().zip(b.iter()) {
    res |= x ^ y;
  }
  Ok(res == 0)
}

#[napi]
pub fn get_hashes() -> Vec<String> {
  vec![
    "sha1".into(),
    "sha256".into(),
    "sha384".into(),
    "sha512".into(),
    "sha512-256".into(),
  ]
}

#[napi]
pub fn get_ciphers() -> Vec<String> {
  vec![
    "aes-128-cbc".into(),
    "aes-256-cbc".into(),
    "aes-128-gcm".into(),
    "aes-256-gcm".into(),
    "chacha20-poly1305".into(),
  ]
}

#[napi]
pub fn get_curves() -> Vec<String> {
  vec!["p256".into(), "p384".into(), "p521".into(), "x25519".into()]
}

#[napi]
pub fn get_macs() -> Vec<String> {
  vec![
    "hmac".into(),
    "cmac".into(),
    "gmac".into(),
    "kmac".into(),
    "poly1305".into(),
    "siphash".into(),
  ]
}

#[derive(Clone, Copy)]
struct OkmAlgo(usize);
impl KeyType for OkmAlgo {
  fn len(&self) -> usize {
    self.0
  }
}

#[napi(
  ts_args_type = "digest: string, ikm: string | Buffer, salt: string | Buffer, info: string | Buffer, keylen: number"
)]
pub fn hkdf_sync(
  digest: String,
  ikm: Either<String, Buffer>,
  salt: Either<String, Buffer>,
  info: Either<String, Buffer>,
  keylen: u32,
) -> napi::Result<Buffer> {
  let algo = match digest.to_lowercase().replace("-", "").as_str() {
    "sha256" => HKDF_SHA256,
    "sha384" => HKDF_SHA384,
    "sha512" => HKDF_SHA512,
    _ => Err(napi::Error::from_reason(format!("Unsupported HKDF digest: {}", digest)))?,
  };

  let ikm_bytes = decode_input(&ikm, None);
  let salt_bytes = decode_input(&salt, None);
  let info_bytes = decode_input(&info, None);

  let salt_obj = Salt::new(algo, &salt_bytes);
  let prk = salt_obj.extract(&ikm_bytes);
  let info_slice: &[&[u8]] = &[&info_bytes];
  let okm = prk
    .expand(info_slice, OkmAlgo(keylen as usize))
    .map_err(|_| napi::Error::from_reason("HKDF expansion failed"))?;

  let mut out = vec![0u8; keylen as usize];
  okm.fill(&mut out)
    .map_err(|_| napi::Error::from_reason("HKDF fill failed"))?;

  Ok(Buffer::from(out))
}
