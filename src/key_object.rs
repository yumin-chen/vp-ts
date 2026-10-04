use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub struct KeyObject {
  pub key_type: String,
  pub asymmetric_key_type: Option<String>,
  raw_key: Vec<u8>,
}

#[napi]
impl KeyObject {
  #[napi(constructor)]
  pub fn new(key_type: String, asymmetric_key_type: Option<String>, raw_key: Buffer) -> Self {
    Self {
      key_type,
      asymmetric_key_type,
      raw_key: raw_key.to_vec(),
    }
  }

  #[napi]
  pub fn export(&self) -> Buffer {
    Buffer::from(self.raw_key.clone())
  }
}

#[napi]
pub fn create_secret_key(key: Buffer) -> KeyObject {
  KeyObject::new("secret".to_string(), None, key)
}
