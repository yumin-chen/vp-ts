use napi::bindgen_prelude::*;
use napi_derive::napi;
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

#[napi]
pub struct HmacHasher {
  key: Vec<u8>,
  buffer: Vec<u8>,
}

#[napi]
impl HmacHasher {
  #[napi(constructor)]
  pub fn new(key: Buffer) -> Self {
    Self {
      key: key.to_vec(),
      buffer: Vec::new(),
    }
  }

  #[napi]
  pub fn update(&mut self, data: Buffer) {
    self.buffer.extend_from_slice(&data);
  }

  #[napi]
  pub fn digest(&mut self) -> String {
    if let Ok(mut mac) = HmacSha256::new_from_slice(&self.key) {
      mac.update(&self.buffer);
      let result = mac.finalize().into_bytes();
      let mut hex_str = String::with_capacity(result.len() * 2);
      for b in result {
        use std::fmt::Write;
        let _ = write!(hex_str, "{:02x}", b);
      }
      hex_str
    } else {
      String::new()
    }
  }
}

#[napi]
pub fn create_hmac(_algorithm: String, key: Buffer) -> HmacHasher {
  HmacHasher::new(key)
}
