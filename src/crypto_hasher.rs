use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::digest;

#[napi]
pub struct Hash {
  ctx: digest::Context,
}

#[napi]
impl Hash {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> Result<Self> {
    let algo = match algorithm.to_lowercase().as_str() {
      "sha1" => &digest::SHA1_FOR_LEGACY_USE_ONLY,
      "sha256" => &digest::SHA256,
      "sha384" => &digest::SHA384,
      "sha512" => &digest::SHA512,
      _ => {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Unsupported digest algorithm: {}", algorithm),
        ))
      }
    };

    Ok(Hash {
      ctx: digest::Context::new(algo),
    })
  }

  #[napi]
  pub fn update(&mut self, data: Buffer) {
    self.ctx.update(&data);
  }

  #[napi]
  pub fn digest(&self, encoding: Option<String>) -> Either<String, Buffer> {
    let d = self.ctx.clone().finish();
    let bytes = d.as_ref();

    match encoding.as_deref() {
      Some("hex") => Either::A(hex::encode(bytes)),
      _ => Either::B(Buffer::from(bytes.to_vec())),
    }
  }
}

#[napi]
pub fn create_hash(algorithm: String) -> Result<Hash> {
  Hash::new(algorithm)
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
