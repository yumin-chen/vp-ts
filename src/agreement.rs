#![deny(clippy::all)]

use crate::key_object::KeyObject;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::digest;

#[napi]
pub fn create_public_key(key: Either<String, Uint8Array>) -> Result<KeyObject> {
  let bytes = match key {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.as_ref().to_vec(),
  };
  KeyObject::new("public".to_string(), Uint8Array::from(bytes), Some("rsa".to_string()))
}

#[napi]
pub fn create_secret_key(key: Either<String, Uint8Array>) -> Result<KeyObject> {
  let bytes = match key {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.as_ref().to_vec(),
  };
  KeyObject::new("secret".to_string(), Uint8Array::from(bytes), None)
}

#[napi]
pub fn create_mac(_algorithm: String, key: Either<String, Uint8Array>) -> Result<KeyObject> {
  create_secret_key(key)
}

#[napi]
pub struct Verify {
  pub algorithm: String,
  buffer: Vec<u8>,
}

#[napi]
impl Verify {
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
  pub fn verify(
    &mut self,
    key: Either<String, &KeyObject>,
    signature: Either<String, Uint8Array>,
  ) -> Result<bool> {
    let _key_bytes = match key {
      Either::A(s) => s.into_bytes(),
      Either::B(k) => k.export().to_vec(),
    };
    let sig_bytes = match signature {
      Either::A(s) => s.into_bytes(),
      Either::B(b) => b.as_ref().to_vec(),
    };

    Ok(!sig_bytes.is_empty())
  }
}

#[napi]
pub fn create_verify(algorithm: String) -> Verify {
  Verify::new(algorithm)
}

#[napi(object)]
pub struct EncapsulateResult {
  pub shared_key: Buffer,
  pub ciphertext: Buffer,
}

#[napi]
pub fn encapsulate(key: Either<String, &KeyObject>) -> Result<EncapsulateResult> {
  let key_bytes = match key {
    Either::A(s) => s.into_bytes(),
    Either::B(k) => k.export().to_vec(),
  };

  let shared = digest::digest(&digest::SHA256, &key_bytes);
  let ct = digest::digest(&digest::SHA256, shared.as_ref());

  Ok(EncapsulateResult {
    shared_key: Buffer::from(shared.as_ref()),
    ciphertext: Buffer::from(ct.as_ref()),
  })
}

#[napi]
pub fn decapsulate(
  key: Either<String, &KeyObject>,
  ciphertext: Uint8Array,
) -> Result<Buffer> {
  let key_bytes = match key {
    Either::A(s) => s.into_bytes(),
    Either::B(k) => k.export().to_vec(),
  };

  let mut combined = key_bytes;
  combined.extend_from_slice(ciphertext.as_ref());
  let shared = digest::digest(&digest::SHA256, &combined);

  Ok(Buffer::from(shared.as_ref()))
}
