#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::hmac;

fn get_hmac_algorithm(name: &str) -> Result<hmac::Algorithm> {
  let normalized = name.to_lowercase().replace(['-', '_'], "");
  match normalized.as_str() {
    "sha1" => Ok(hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY),
    "sha256" => Ok(hmac::HMAC_SHA256),
    "sha384" => Ok(hmac::HMAC_SHA384),
    "sha512" => Ok(hmac::HMAC_SHA512),
    _ => Err(Error::new(
      Status::InvalidArg,
      format!("Unsupported HMAC algorithm: {name}"),
    )),
  }
}

#[napi]
pub struct Hmac {
  context: Option<hmac::Context>,
}

#[napi]
impl Hmac {
  #[napi(constructor)]
  pub fn new(algorithm: String, key: Either<String, Uint8Array>) -> Result<Self> {
    let alg = get_hmac_algorithm(&algorithm)?;
    let key_bytes = match key {
      Either::A(s) => s.into_bytes(),
      Either::B(b) => b.as_ref().to_vec(),
    };
    let s_key = hmac::Key::new(alg, &key_bytes);
    let ctx = hmac::Context::with_key(&s_key);

    Ok(Self { context: Some(ctx) })
  }

  #[napi]
  pub fn update(
    &mut self,
    data: Either<String, Uint8Array>,
    _encoding: Option<String>,
  ) -> Result<()> {
    let ctx = self
      .context
      .as_mut()
      .ok_or_else(|| Error::new(Status::GenericFailure, "Digest already called"))?;

    match data {
      Either::A(s) => ctx.update(s.as_bytes()),
      Either::B(b) => ctx.update(b.as_ref()),
    }
    Ok(())
  }

  #[napi]
  pub fn digest(&mut self, encoding: Option<String>) -> Result<Either<String, Buffer>> {
    let ctx = self
      .context
      .take()
      .ok_or_else(|| Error::new(Status::GenericFailure, "Digest already called"))?;

    let tag = ctx.sign();
    let bytes = tag.as_ref();

    let enc = encoding.unwrap_or_else(|| "buffer".to_string());
    match enc.to_lowercase().as_str() {
      "hex" => {
        let hex_str = bytes.iter().map(|b| format!("{b:02x}")).collect::<String>();
        Ok(Either::A(hex_str))
      }
      "base64" => {
        let b64 = base64_encode(bytes);
        Ok(Either::A(b64))
      }
      "latin1" | "binary" => {
        let latin1_str = bytes.iter().map(|&b| b as char).collect::<String>();
        Ok(Either::A(latin1_str))
      }
      "buffer" | "" => Ok(Either::B(Buffer::from(bytes))),
      _ => Err(Error::new(
        Status::InvalidArg,
        format!("Unknown digest encoding: {enc}"),
      )),
    }
  }
}

fn base64_encode(data: &[u8]) -> String {
  const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
  let mut result = String::new();
  let mut i = 0;
  while i < data.len() {
    let b0 = data[i] as u32;
    let b1 = if i + 1 < data.len() { data[i + 1] as u32 } else { 0 };
    let b2 = if i + 2 < data.len() { data[i + 2] as u32 } else { 0 };

    let triplet = (b0 << 16) | (b1 << 8) | b2;

    result.push(CHARS[((triplet >> 18) & 0x3F) as usize] as char);
    result.push(CHARS[((triplet >> 12) & 0x3F) as usize] as char);

    if i + 1 < data.len() {
      result.push(CHARS[((triplet >> 6) & 0x3F) as usize] as char);
    } else {
      result.push('=');
    }

    if i + 2 < data.len() {
      result.push(CHARS[(triplet & 0x3F) as usize] as char);
    } else {
      result.push('=');
    }

    i += 3;
  }
  result
}

#[napi]
pub fn create_hmac(algorithm: String, key: Either<String, Uint8Array>) -> Result<Hmac> {
  Hmac::new(algorithm, key)
}
