#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::digest;

#[napi]
pub struct CryptoHasher {
  context: Option<digest::Context>,
}

#[napi]
impl CryptoHasher {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> Result<Self> {
    let norm = algorithm.to_lowercase().replace(['-', '_'], "");
    let alg: &'static digest::Algorithm = match norm.as_str() {
      "sha1" => &digest::SHA1_FOR_LEGACY_USE_ONLY,
      "sha256" => &digest::SHA256,
      "sha384" => &digest::SHA384,
      "sha512" => &digest::SHA512,
      _ => {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Unsupported algorithm for CryptoHasher: {algorithm}"),
        ))
      }
    };

    let ctx = digest::Context::new(alg);
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

    let output = ctx.finish();
    let bytes = output.as_ref();

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
pub fn create_hash(algorithm: String) -> Result<CryptoHasher> {
  CryptoHasher::new(algorithm)
}

#[napi]
pub fn crypto_hash(
  algorithm: String,
  data: Either<String, Uint8Array>,
  encoding: Option<String>,
) -> Result<Either<String, Buffer>> {
  let mut hasher = CryptoHasher::new(algorithm)?;
  hasher.update(data, None)?;
  hasher.digest(encoding)
}

#[napi(js_name = "getHashes")]
pub fn get_hashes() -> Vec<String> {
  vec![
    "sha1".to_string(),
    "sha256".to_string(),
    "sha384".to_string(),
    "sha512".to_string(),
  ]
}

#[napi(js_name = "getMacs")]
pub fn get_macs() -> Vec<String> {
  vec!["hmac".to_string()]
}

#[napi(js_name = "getCiphers")]
pub fn get_ciphers() -> Vec<String> {
  vec!["aes-256-gcm".to_string(), "aes-128-ccm".to_string()]
}

#[napi(js_name = "getCurves")]
pub fn get_curves() -> Vec<String> {
  vec!["prime256v1".to_string(), "secp256k1".to_string()]
}

#[napi(js_name = "timingSafeEqual")]
pub fn timing_safe_equal(a: Uint8Array, b: Uint8Array) -> Result<bool> {
  if a.len() != b.len() {
    return Err(Error::new(
      Status::InvalidArg,
      "Input buffers must have the same length",
    ));
  }
  let mut diff = 0u8;
  for (&x, &y) in a.as_ref().iter().zip(b.as_ref().iter()) {
    diff |= x ^ y;
  }
  Ok(diff == 0)
}
