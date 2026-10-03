use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::digest;

#[napi]
pub struct Hasher {
  pub algorithm: String,
  context: Option<digest::Context>,
}

#[napi]
impl Hasher {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> Result<Self> {
    let context = Self::get_context(&algorithm)?;
    Ok(Self {
      algorithm: algorithm.to_lowercase(),
      context: Some(context),
    })
  }

  fn get_context(algorithm: &str) -> Result<digest::Context> {
    let lower = algorithm.to_lowercase();
    let algo = match lower.as_str() {
      "sha256" => &digest::SHA256,
      "sha384" => &digest::SHA384,
      "sha512" => &digest::SHA512,
      "sha512_256" => &digest::SHA512_256,
      "sha1" => &digest::SHA1_FOR_LEGACY_USE_ONLY,
      _ => {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Digest method not supported: {}", algorithm),
        ))
      }
    };
    Ok(digest::Context::new(algo))
  }

  #[napi]
  pub fn update(&mut self, data: Either<String, Buffer>) -> Result<&Self> {
    match &mut self.context {
      Some(ctx) => {
        match data {
          Either::A(s) => ctx.update(s.as_bytes()),
          Either::B(b) => ctx.update(b.as_ref()),
        }
        Ok(self)
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

    let result = ctx.finish();
    let bytes = result.as_ref();

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
pub fn create_hash(algorithm: String) -> Result<Hasher> {
  Hasher::new(algorithm)
}

#[napi]
pub fn hash(algorithm: String, data: Either<String, Buffer>, encoding: Option<String>) -> Result<Either<String, Buffer>> {
  let mut hasher = Hasher::new(algorithm)?;
  hasher.update(data)?;
  hasher.digest(encoding)
}

#[napi]
pub fn get_hashes() -> Vec<String> {
  vec![
    "sha256".to_string(),
    "sha384".to_string(),
    "sha512".to_string(),
    "sha512_256".to_string(),
    "sha1".to_string(),
  ]
}
