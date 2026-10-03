use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::digest;

#[napi]
pub struct Hash {
  ctx: Option<digest::Context>,
  #[allow(dead_code)]
  algorithm: String,
}

#[napi]
impl Hash {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> Result<Self> {
    let algo = match algorithm.to_lowercase().replace('-', "").as_str() {
      "sha1" => &digest::SHA1_FOR_LEGACY_USE_ONLY,
      "sha256" => &digest::SHA256,
      "sha384" => &digest::SHA384,
      "sha512" => &digest::SHA512,
      "sha512_256" => &digest::SHA512_256,
      _ => {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Unsupported hash algorithm: {}", algorithm),
        ))
      }
    };

    let ctx = digest::Context::new(algo);

    Ok(Self {
      ctx: Some(ctx),
      algorithm,
    })
  }

  #[napi]
  pub fn update(
    &mut self,
    #[napi(ts_arg_type = "string | Uint8Array")] data: Either<String, Uint8Array>,
  ) -> Result<()> {
    let ctx = self
      .ctx
      .as_mut()
      .ok_or_else(|| Error::new(Status::GenericFailure, "Digest already called"))?;
    match &data {
      Either::A(s) => ctx.update(s.as_bytes()),
      Either::B(b) => ctx.update(b.as_ref()),
    }
    Ok(())
  }

  #[napi(ts_return_type = "Buffer | string")]
  pub fn digest(&mut self, encoding: Option<String>) -> Result<Either<Buffer, String>> {
    let ctx = self
      .ctx
      .take()
      .ok_or_else(|| Error::new(Status::GenericFailure, "Digest already called"))?;
    let d = ctx.finish();
    let bytes = d.as_ref();

    match encoding.as_deref() {
      Some("hex") => Ok(Either::B(hex::encode(bytes))),
      Some("base64") => {
        use base64::Engine;
        Ok(Either::B(
          base64::engine::general_purpose::STANDARD.encode(bytes),
        ))
      }
      Some("buffer") | None => Ok(Either::A(Buffer::from(bytes))),
      Some(other) => Err(Error::new(
        Status::InvalidArg,
        format!("Unsupported encoding: {}", other),
      )),
    }
  }
}

#[napi]
pub fn create_hash(algorithm: String) -> Result<Hash> {
  Hash::new(algorithm)
}

#[napi(ts_return_type = "Buffer | string")]
pub fn hash(
  algorithm: String,
  #[napi(ts_arg_type = "string | Uint8Array")] data: Either<String, Uint8Array>,
  encoding: Option<String>,
) -> Result<Either<Buffer, String>> {
  let mut hasher = Hash::new(algorithm)?;
  hasher.update(data)?;
  hasher.digest(encoding)
}

#[napi]
pub fn get_hashes() -> Vec<String> {
  vec![
    "sha1".to_string(),
    "sha256".to_string(),
    "sha384".to_string(),
    "sha512".to_string(),
    "sha512_256".to_string(),
  ]
}
