use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::digest::{Context, SHA1_FOR_LEGACY_USE_ONLY, SHA256, SHA384, SHA512};

fn get_digest_algorithm(alg: &str) -> Result<&'static ring::digest::Algorithm> {
  match alg.to_lowercase().replace("-", "").as_str() {
    "sha1" => Ok(&SHA1_FOR_LEGACY_USE_ONLY),
    "sha256" => Ok(&SHA256),
    "sha384" => Ok(&SHA384),
    "sha512" => Ok(&SHA512),
    _ => Err(Error::from_reason(format!("Unsupported digest algorithm: {alg}"))),
  }
}

fn to_hex(bytes: &[u8]) -> String {
  bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

fn to_base64(bytes: &[u8]) -> String {
  use base64::Engine;
  base64::engine::general_purpose::STANDARD.encode(bytes)
}

#[napi]
pub struct CryptoHasher {
  ctx: Option<Context>,
}

#[napi]
impl CryptoHasher {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> Result<Self> {
    let alg = get_digest_algorithm(&algorithm)?;
    let ctx = Context::new(alg);
    Ok(CryptoHasher { ctx: Some(ctx) })
  }

  #[napi]
  pub fn update(&mut self, data: Either<String, Buffer>) -> Result<()> {
    let ctx = self.ctx.as_mut().ok_or_else(|| {
      Error::from_reason("ERR_CRYPTO_HASH_FINALIZED: Digest already called")
    })?;
    let bytes = match data {
      Either::A(s) => s.into_bytes(),
      Either::B(b) => b.to_vec(),
    };
    ctx.update(&bytes);
    Ok(())
  }

  #[napi]
  pub fn digest(&mut self, encoding: Option<String>) -> Result<Either<String, Buffer>> {
    let ctx = self.ctx.take().ok_or_else(|| {
      Error::from_reason("ERR_CRYPTO_HASH_FINALIZED: Digest already called")
    })?;
    let digest = ctx.finish();
    let bytes = digest.as_ref();
    match encoding.as_deref() {
      Some("hex") => Ok(Either::A(to_hex(bytes))),
      Some("base64") => Ok(Either::A(to_base64(bytes))),
      _ => Ok(Either::B(Buffer::from(bytes.to_vec()))),
    }
  }
}

#[napi]
pub fn get_hashes() -> Vec<String> {
  vec![
    "sha1".to_string(),
    "sha256".to_string(),
    "sha384".to_string(),
    "sha512".to_string(),
  ]
}

#[napi]
pub fn hash(
  algorithm: String,
  data: Either<String, Buffer>,
  output_encoding: Option<String>,
) -> Result<Either<String, Buffer>> {
  let mut hasher = CryptoHasher::new(algorithm)?;
  hasher.update(data)?;
  hasher.digest(output_encoding)
}
