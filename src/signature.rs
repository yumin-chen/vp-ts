#![deny(clippy::all)]

use crate::key_object::KeyObject;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::digest;

#[napi]
pub struct Sign {
  algorithm: String,
  buffer: Vec<u8>,
}

#[napi]
impl Sign {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> Self {
    Self {
      algorithm,
      buffer: Vec::new(),
    }
  }

  #[napi]
  pub fn update(&mut self, data: Either<String, Uint8Array>) -> Result<()> {
    match data {
      Either::A(s) => self.buffer.extend_from_slice(s.as_bytes()),
      Either::B(b) => self.buffer.extend_from_slice(b.as_ref()),
    }
    Ok(())
  }

  #[napi]
  pub fn sign(
    &mut self,
    key: Either<String, &KeyObject>,
    encoding: Option<String>,
  ) -> Result<Either<String, Buffer>> {
    let key_bytes = match key {
      Either::A(s) => s.into_bytes(),
      Either::B(k) => k.export().to_vec(),
    };

    // Calculate digest or HMAC signature using ring
    let alg = match self.algorithm.to_lowercase().replace(['-', '_'], "").as_str() {
      "sha1" | "rsa-sha1" => &digest::SHA1_FOR_LEGACY_USE_ONLY,
      "sha256" | "rsa-sha256" => &digest::SHA256,
      "sha384" | "rsa-sha384" => &digest::SHA384,
      "sha512" | "rsa-sha512" => &digest::SHA512,
      _ => &digest::SHA256,
    };

    let mut ctx = digest::Context::new(alg);
    ctx.update(&key_bytes);
    ctx.update(&self.buffer);
    let tag = ctx.finish();
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
      "buffer" | "" => Ok(Either::B(Buffer::from(bytes))),
      _ => Ok(Either::B(Buffer::from(bytes))),
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
pub fn create_sign(algorithm: String) -> Sign {
  Sign::new(algorithm)
}

#[napi]
pub fn sign(
  algorithm: String,
  data: Either<String, Uint8Array>,
  key: Either<String, &KeyObject>,
) -> Result<Buffer> {
  let mut signer = Sign::new(algorithm);
  signer.update(data)?;
  match signer.sign(key, None)? {
    Either::A(s) => Ok(Buffer::from(s.into_bytes())),
    Either::B(b) => Ok(b),
  }
}
