use napi::bindgen_prelude::*;
use napi_derive::napi;
use sha2::{Digest, Sha256, Sha512};

#[napi]
pub struct CryptoHasher {
  algorithm: String,
  buffer: Vec<u8>,
}

#[napi]
impl CryptoHasher {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> Self {
    Self {
      algorithm,
      buffer: Vec::new(),
    }
  }

  #[napi]
  pub fn update(&mut self, data: Buffer) {
    self.buffer.extend_from_slice(&data);
  }

  #[napi]
  pub fn digest(&mut self, _encoding: Option<String>) -> String {
    let bytes = if self.algorithm.to_lowercase() == "sha512" {
      let mut hasher = Sha512::new();
      hasher.update(&self.buffer);
      hasher.finalize().to_vec()
    } else {
      let mut hasher = Sha256::new();
      hasher.update(&self.buffer);
      hasher.finalize().to_vec()
    };

    let mut hex_str = String::with_capacity(bytes.len() * 2);
    for b in bytes {
      use std::fmt::Write;
      let _ = write!(hex_str, "{:02x}", b);
    }
    hex_str
  }
}

#[napi]
pub fn create_hash(algorithm: String) -> CryptoHasher {
  CryptoHasher::new(algorithm)
}
