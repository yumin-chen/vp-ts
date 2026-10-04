use base64ct::{Base64, Encoding};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::hmac;

#[napi]
pub struct Hmac {
  key: Vec<u8>,
  algorithm: String,
  data: Vec<u8>,
}

#[napi]
impl Hmac {
  #[napi(constructor)]
  pub fn new(algorithm: String, key: Either<Buffer, String>) -> Result<Self> {
    let key_bytes = match key {
      Either::A(b) => b.as_ref().to_vec(),
      Either::B(s) => s.as_bytes().to_vec(),
    };
    Ok(Hmac {
      key: key_bytes,
      algorithm,
      data: Vec::new(),
    })
  }

  #[napi]
  pub fn update(&mut self, data: Either<Buffer, String>) -> Result<()> {
    match data {
      Either::A(b) => self.data.extend_from_slice(b.as_ref()),
      Either::B(s) => self.data.extend_from_slice(s.as_bytes()),
    }
    Ok(())
  }

  #[napi]
  pub fn digest(&mut self, encoding: Option<String>) -> Result<Either<Buffer, String>> {
    let algorithm_lower = self.algorithm.to_lowercase().replace('-', "");
    let ring_alg = match algorithm_lower.as_str() {
      "sha256" => hmac::HMAC_SHA256,
      "sha384" => hmac::HMAC_SHA384,
      "sha512" => hmac::HMAC_SHA512,
      "sha1" => hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY,
      other => return Err(Error::from_reason(format!("Unsupported HMAC algorithm: {other}"))),
    };

    let s_key = hmac::Key::new(ring_alg, &self.key);
    let tag = hmac::sign(&s_key, &self.data);
    let bytes = tag.as_ref();

    match encoding.as_deref() {
      Some("hex") => Ok(Either::B(hex::encode(bytes))),
      Some("base64") => Ok(Either::B(Base64::encode_string(bytes))),
      _ => Ok(Either::A(Buffer::from(bytes))),
    }
  }
}

#[napi(js_name = "createHmac")]
pub fn create_hmac(algorithm: String, key: Either<Buffer, String>) -> Result<Hmac> {
  Hmac::new(algorithm, key)
}
