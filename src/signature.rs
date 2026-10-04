use napi::bindgen_prelude::*;
use napi_derive::napi;
use sha2::{Digest, Sha256};

#[napi]
pub struct Signer {
  buffer: Vec<u8>,
}

#[napi]
impl Signer {
  #[napi(constructor)]
  pub fn new(_algorithm: String) -> Self {
    Self {
      buffer: Vec::new(),
    }
  }

  #[napi]
  pub fn update(&mut self, data: Buffer) {
    self.buffer.extend_from_slice(&data);
  }

  #[napi]
  pub fn sign(&self, _private_key: Buffer) -> Buffer {
    let mut hasher = Sha256::new();
    hasher.update(&self.buffer);
    let digest = hasher.finalize();
    Buffer::from(digest.to_vec())
  }
}

#[napi]
pub fn create_sign(algorithm: String) -> Signer {
  Signer::new(algorithm)
}
