use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::hmac;

#[napi]
pub struct Hmac {
  ctx: Option<hmac::Context>,
  #[allow(dead_code)]
  algorithm: String,
}

#[napi]
impl Hmac {
  #[napi(constructor)]
  pub fn new(
    algorithm: String,
    #[napi(ts_arg_type = "string | Uint8Array")] key: Either<String, Uint8Array>,
  ) -> Result<Self> {
    let key_bytes = match &key {
      Either::A(s) => s.as_bytes(),
      Either::B(b) => b.as_ref(),
    };
    let algo = match algorithm.to_lowercase().replace('-', "").as_str() {
      "sha1" => hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY,
      "sha256" => hmac::HMAC_SHA256,
      "sha384" => hmac::HMAC_SHA384,
      "sha512" => hmac::HMAC_SHA512,
      _ => {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Unsupported HMAC algorithm: {}", algorithm),
        ))
      }
    };

    let s_key = hmac::Key::new(algo, key_bytes);
    let ctx = hmac::Context::with_key(&s_key);

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
    let tag = ctx.sign();
    let bytes = tag.as_ref();

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
pub fn create_hmac(
  algorithm: String,
  #[napi(ts_arg_type = "string | Uint8Array")] key: Either<String, Uint8Array>,
) -> Result<Hmac> {
  Hmac::new(algorithm, key)
}
