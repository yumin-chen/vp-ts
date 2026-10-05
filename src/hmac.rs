use base64::Engine;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::hmac;

#[napi]
pub struct Hmac {
  ctx: hmac::Context,
}

#[napi]
impl Hmac {
  #[napi(constructor)]
  pub fn new(algorithm: String, key: Buffer) -> Result<Self> {
    let algo = match algorithm.to_lowercase().as_str() {
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

    let key_obj = hmac::Key::new(algo, &key);
    let ctx = hmac::Context::with_key(&key_obj);

    Ok(Hmac { ctx })
  }

  #[napi]
  pub fn update(&mut self, data: Buffer) {
    self.ctx.update(&data);
  }

  #[napi]
  pub fn digest(&self, encoding: Option<String>) -> Result<Either<String, Buffer>> {
    let tag = self.ctx.clone().sign();
    let bytes = tag.as_ref();

    match encoding.as_deref() {
      Some("hex") => Ok(Either::A(hex::encode(bytes))),
      Some("base64") => Ok(Either::A(
        base64::engine::general_purpose::STANDARD.encode(bytes),
      )),
      _ => Ok(Either::B(Buffer::from(bytes.to_vec()))),
    }
  }
}

#[napi]
pub fn create_hmac(algorithm: String, key: Buffer) -> Result<Hmac> {
  Hmac::new(algorithm, key)
}
