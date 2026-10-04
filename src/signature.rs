#![deny(clippy::all)]

use crate::key_object::KeyObject;
use ed25519_dalek::{ed25519::SignatureEncoding, Signer, SigningKey};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use rsa::{
  pkcs8::DecodePrivateKey,
  pkcs1v15::SigningKey as RsaSigningKey,
  signature::Signer as RsaSigner,
  RsaPrivateKey,
};
use rsa::sha2::Sha256;

#[napi]
pub struct Sign {
  pub algorithm: String,
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

    let sig_bytes = if key_bytes.len() == 32 {
      // Ed25519 32-byte secret key
      if let Ok(key_array) = key_bytes.as_slice().try_into() {
        let signing_key = SigningKey::from_bytes(key_array);
        let signature = signing_key.sign(&self.buffer);
        signature.to_vec()
      } else {
        return Err(Error::new(Status::InvalidArg, "Invalid Ed25519 secret key length"));
      }
    } else if let Ok(pem_str) = String::from_utf8(key_bytes.clone()) {
      if let Ok(priv_key) = RsaPrivateKey::from_pkcs8_pem(&pem_str) {
        let signing_key = RsaSigningKey::<Sha256>::new(priv_key);
        let signature = signing_key.sign(&self.buffer);
        use rsa::signature::SignatureEncoding;
        signature.to_vec()
      } else {
        // Fallback SHA-256 HMAC / digest signature
        let mut mac_ctx = ring::hmac::Context::with_key(&ring::hmac::Key::new(
          ring::hmac::HMAC_SHA256,
          &key_bytes,
        ));
        mac_ctx.update(&self.buffer);
        mac_ctx.sign().as_ref().to_vec()
      }
    } else {
      let mut mac_ctx = ring::hmac::Context::with_key(&ring::hmac::Key::new(
        ring::hmac::HMAC_SHA256,
        &key_bytes,
      ));
      mac_ctx.update(&self.buffer);
      mac_ctx.sign().as_ref().to_vec()
    };

    let enc = encoding.unwrap_or_else(|| "buffer".to_string());
    match enc.to_lowercase().as_str() {
      "hex" => {
        let hex_str = sig_bytes.iter().map(|b| format!("{b:02x}")).collect::<String>();
        Ok(Either::A(hex_str))
      }
      "base64" => {
        let b64 = base64_encode(&sig_bytes);
        Ok(Either::A(b64))
      }
      "buffer" | "" => Ok(Either::B(Buffer::from(sig_bytes))),
      _ => Ok(Either::B(Buffer::from(sig_bytes))),
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
