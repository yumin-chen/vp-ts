#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::hmac;

fn get_hmac_algorithm(algo: &str) -> Result<hmac::Algorithm> {
  let normalized = algo.to_lowercase().replace('-', "");
  match normalized.as_str() {
    "sha1" => Ok(hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY),
    "sha256" => Ok(hmac::HMAC_SHA256),
    "sha384" => Ok(hmac::HMAC_SHA384),
    "sha512" => Ok(hmac::HMAC_SHA512),
    _ => Err(Error::new(
      Status::InvalidArg,
      format!("Digest method not supported: {algo}"),
    )),
  }
}

fn parse_string_encoding(s: &str, encoding: Option<&str>) -> Result<Vec<u8>> {
  match encoding.map(|e| e.to_lowercase()).as_deref() {
    Some("hex") => {
      let mut vec = Vec::new();
      let mut bytes = s.bytes();
      while let (Some(h), Some(l)) = (bytes.next(), bytes.next()) {
        let h_val = (h as char)
          .to_digit(16)
          .ok_or_else(|| Error::new(Status::InvalidArg, "Invalid hex string"))?;
        let l_val = (l as char)
          .to_digit(16)
          .ok_or_else(|| Error::new(Status::InvalidArg, "Invalid hex string"))?;
        vec.push(((h_val << 4) | l_val) as u8);
      }
      Ok(vec)
    }
    Some("base64") => Ok(s.as_bytes().to_vec()),
    _ => Ok(s.as_bytes().to_vec()),
  }
}

#[napi(js_name = "Hmac")]
pub struct Hmac {
  context: Option<hmac::Context>,
}

#[napi]
impl Hmac {
  #[napi(constructor)]
  pub fn new(algorithm: String, key: Either<String, &[u8]>) -> Result<Self> {
    let algo = get_hmac_algorithm(&algorithm)?;
    let key_bytes = match key {
      Either::A(ref s) => s.as_bytes(),
      Either::B(b) => b,
    };
    let s_key = hmac::Key::new(algo, key_bytes);
    let context = hmac::Context::with_key(&s_key);
    Ok(Self {
      context: Some(context),
    })
  }

  #[napi]
  pub fn update(
    &mut self,
    data: Either<String, &[u8]>,
    encoding: Option<String>,
  ) -> Result<&Self> {
    let ctx = self.context.as_mut().ok_or_else(|| {
      Error::new(
        Status::GenericFailure,
        "HMAC object has already been finalized",
      )
    })?;

    match data {
      Either::A(s) => {
        let bytes = parse_string_encoding(&s, encoding.as_deref())?;
        ctx.update(&bytes);
      }
      Either::B(b) => {
        ctx.update(b);
      }
    }
    Ok(self)
  }

  #[napi]
  pub fn digest(&mut self, encoding: Option<String>) -> Result<Either<String, Buffer>> {
    let ctx = self.context.take().ok_or_else(|| {
      Error::new(
        Status::GenericFailure,
        "Digest already called on HMAC object",
      )
    })?;

    let tag = ctx.sign();
    let bytes = tag.as_ref();

    match encoding.map(|e| e.to_lowercase()).as_deref() {
      Some("hex") => {
        let hex_str: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        Ok(Either::A(hex_str))
      }
      Some("base64") => {
        let mut base64_str = String::new();
        const CHARS: &[u8] =
          b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut i = 0;
        while i < bytes.len() {
          let b0 = bytes[i];
          let b1 = if i + 1 < bytes.len() { bytes[i + 1] } else { 0 };
          let b2 = if i + 2 < bytes.len() { bytes[i + 2] } else { 0 };

          base64_str.push(CHARS[(b0 >> 2) as usize] as char);
          base64_str.push(CHARS[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);
          if i + 1 < bytes.len() {
            base64_str.push(CHARS[(((b1 & 0x0f) << 2) | (b2 >> 6)) as usize] as char);
          } else {
            base64_str.push('=');
          }
          if i + 2 < bytes.len() {
            base64_str.push(CHARS[(b2 & 0x3f) as usize] as char);
          } else {
            base64_str.push('=');
          }
          i += 3;
        }
        Ok(Either::A(base64_str))
      }
      _ => Ok(Either::B(Buffer::from(bytes))),
    }
  }
}

#[napi]
pub fn create_hmac(algorithm: String, key: Either<String, &[u8]>) -> Result<Hmac> {
  Hmac::new(algorithm, key)
}
