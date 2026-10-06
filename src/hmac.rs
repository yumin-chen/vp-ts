use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::hmac::{Algorithm, Context, Key, HMAC_SHA1_FOR_LEGACY_USE_ONLY, HMAC_SHA256, HMAC_SHA384, HMAC_SHA512};

fn get_algorithm(algorithm: &str) -> Result<Algorithm> {
  match algorithm.to_lowercase().replace("-", "").as_str() {
    "sha1" => Ok(HMAC_SHA1_FOR_LEGACY_USE_ONLY),
    "sha256" => Ok(HMAC_SHA256),
    "sha384" => Ok(HMAC_SHA384),
    "sha512" => Ok(HMAC_SHA512),
    _ => Err(Error::from_reason(format!("Unsupported HMAC algorithm: {algorithm}"))),
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
pub struct Hmac {
  key: Option<Key>,
  data: Vec<u8>,
}

#[napi]
impl Hmac {
  #[napi(constructor)]
  pub fn new(algorithm: String, key: Either<String, Buffer>) -> Result<Self> {
    let alg = get_algorithm(&algorithm)?;
    let key_bytes = match key {
      Either::A(s) => s.into_bytes(),
      Either::B(b) => b.to_vec(),
    };
    let hmac_key = Key::new(alg, &key_bytes);
    Ok(Hmac {
      key: Some(hmac_key),
      data: vec![],
    })
  }

  #[napi]
  pub fn update(&mut self, data: Either<String, Buffer>) -> Result<()> {
    if self.key.is_none() {
      return Err(Error::from_reason("ERR_CRYPTO_HASH_FINALIZED: Digest already called"));
    }
    let bytes = match data {
      Either::A(s) => s.into_bytes(),
      Either::B(b) => b.to_vec(),
    };
    self.data.extend(bytes);
    Ok(())
  }

  #[napi]
  pub fn digest(&mut self, encoding: Option<String>) -> Result<Either<String, Buffer>> {
    let key = self.key.take().ok_or_else(|| {
      Error::from_reason("ERR_CRYPTO_HASH_FINALIZED: Digest already called")
    })?;
    let mut ctx = Context::with_key(&key);
    ctx.update(&self.data);
    let tag = ctx.sign();
    let bytes = tag.as_ref();
    match encoding.as_deref() {
      Some("hex") => Ok(Either::A(to_hex(bytes))),
      Some("base64") => Ok(Either::A(to_base64(bytes))),
      _ => Ok(Either::B(Buffer::from(bytes.to_vec()))),
    }
  }
}

#[napi]
pub fn create_hmac(algorithm: String, key: Either<String, Buffer>) -> Result<Hmac> {
  Hmac::new(algorithm, key)
}
