use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::hmac;

#[napi]
pub struct Hmac {
  pub algorithm: String,
  context: Option<hmac::Context>,
}

#[napi]
impl Hmac {
  #[napi(constructor)]
  pub fn new(algorithm: String, key: Buffer) -> Result<Self> {
    let lower = algorithm.to_lowercase();
    let algo = match lower.as_str() {
      "sha256" => hmac::HMAC_SHA256,
      "sha384" => hmac::HMAC_SHA384,
      "sha512" => hmac::HMAC_SHA512,
      "sha1" => hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY,
      _ => {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Unknown or unsupported HMAC algorithm: {}", algorithm),
        ))
      }
    };

    let s_key = hmac::Key::new(algo, key.as_ref());
    let ctx = hmac::Context::with_key(&s_key);

    Ok(Self {
      algorithm: lower,
      context: Some(ctx),
    })
  }

  #[napi]
  pub fn update(&mut self, data: Buffer) -> Result<()> {
    match &mut self.context {
      Some(ctx) => {
        ctx.update(data.as_ref());
        Ok(())
      }
      None => Err(Error::new(
        Status::GenericFailure,
        "Cannot update after digest has been called",
      )),
    }
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
    match enc.as_str() {
      "hex" => Ok(Either::A(hex_encode(bytes))),
      "base64" => Ok(Either::A(base64_encode(bytes))),
      "buffer" | "" => Ok(Either::B(Buffer::from(bytes.to_vec()))),
      other => Err(Error::new(
        Status::InvalidArg,
        format!("Unknown encoding: {}", other),
      )),
    }
  }
}

fn hex_encode(data: &[u8]) -> String {
  data.iter().map(|b| format!("{:02x}", b)).collect()
}

fn base64_encode(data: &[u8]) -> String {
  const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
  let mut result = String::new();
  let mut i = 0;
  while i < data.len() {
    let b0 = data[i] as u32;
    let b1 = if i + 1 < data.len() { data[i + 1] as u32 } else { 0 };
    let b2 = if i + 2 < data.len() { data[i + 2] as u32 } else { 0 };

    let triple = (b0 << 16) | (b1 << 8) | b2;

    result.push(ALPHABET[((triple >> 18) & 63) as usize] as char);
    result.push(ALPHABET[((triple >> 12) & 63) as usize] as char);

    if i + 1 < data.len() {
      result.push(ALPHABET[((triple >> 6) & 63) as usize] as char);
    } else {
      result.push('=');
    }

    if i + 2 < data.len() {
      result.push(ALPHABET[(triple & 63) as usize] as char);
    } else {
      result.push('=');
    }

    i += 3;
  }
  result
}

#[napi]
pub fn create_hmac(algorithm: String, key: Buffer) -> Result<Hmac> {
  Hmac::new(algorithm, key)
}
