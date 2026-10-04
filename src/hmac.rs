use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::hmac;

#[napi]
pub struct Hmac {
  context: hmac::Context,
}

#[napi]
impl Hmac {
  #[napi(constructor)]
  pub fn new(algorithm: String, key: Buffer) -> Result<Self> {
    let algo_clean = algorithm.to_lowercase().replace('-', "");
    let algo = match algo_clean.as_str() {
      "sha1" => hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY,
      "sha256" => hmac::HMAC_SHA256,
      "sha384" => hmac::HMAC_SHA384,
      "sha512" => hmac::HMAC_SHA512,
      _ => return Err(Error::new(Status::InvalidArg, format!("Unknown message digest: {}", algorithm))),
    };

    let key = hmac::Key::new(algo, &key);
    let context = hmac::Context::with_key(&key);
    Ok(Self { context })
  }

  #[napi]
  pub fn update(&mut self, data: Buffer) -> Result<()> {
    self.context.update(&data);
    Ok(())
  }

  #[napi]
  pub fn digest(&mut self, encoding: Option<String>) -> Result<Either<Buffer, String>> {
    let dummy_key = hmac::Key::new(hmac::HMAC_SHA256, &[]);
    let ctx = std::mem::replace(&mut self.context, hmac::Context::with_key(&dummy_key));
    let tag = ctx.sign();
    let bytes = tag.as_ref().to_vec();

    match encoding.as_deref() {
      Some("hex") => Ok(Either::B(hex::encode(bytes))),
      Some("base64") => {
        use base64ct::{Base64, Encoding};
        Ok(Either::B(Base64::encode_string(&bytes)))
      }
      _ => Ok(Either::A(Buffer::from(bytes))),
    }
  }
}

#[napi]
pub fn create_hmac(algorithm: String, key: Buffer) -> Result<Hmac> {
  Hmac::new(algorithm, key)
}

#[napi]
pub fn get_macs() -> Vec<String> {
  vec!["hmac".to_string(), "cmac".to_string(), "gmac".to_string()]
}
