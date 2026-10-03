use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
#[derive(Clone)]
pub struct KeyObject {
  pub key_type: String, // "secret", "public", "private"
  pub raw_data: Vec<u8>,
}

#[napi]
impl KeyObject {
  #[napi(constructor)]
  pub fn new(key_type: String, raw_data: Buffer) -> Self {
    Self {
      key_type,
      raw_data: raw_data.as_ref().to_vec(),
    }
  }

  #[napi]
  pub fn export(&self) -> Buffer {
    Buffer::from(self.raw_data.clone())
  }
}

#[napi]
pub fn create_secret_key(key: Buffer) -> KeyObject {
  KeyObject::new("secret".to_string(), key)
}

#[napi]
pub fn create_public_key(key: Either<String, Buffer>) -> KeyObject {
  let bytes = match key {
    Either::A(s) => Buffer::from(s.into_bytes()),
    Either::B(b) => b,
  };
  KeyObject::new("public".to_string(), bytes)
}

#[napi]
pub fn create_private_key(key: Either<String, Buffer>) -> KeyObject {
  let bytes = match key {
    Either::A(s) => Buffer::from(s.into_bytes()),
    Either::B(b) => b,
  };
  KeyObject::new("private".to_string(), bytes)
}

pub struct X509CertificateData {
  pub raw_data: Vec<u8>,
}

#[napi]
pub struct X509Certificate {
  data: X509CertificateData,
}

#[napi]
impl X509Certificate {
  #[napi(constructor)]
  pub fn new(buffer: Buffer) -> Self {
    Self {
      data: X509CertificateData {
        raw_data: buffer.as_ref().to_vec(),
      },
    }
  }

  #[napi(getter)]
  pub fn raw(&self) -> Buffer {
    Buffer::from(self.data.raw_data.clone())
  }

  #[napi]
  pub fn to_string(&self) -> String {
    format!("[X509Certificate: {} bytes]", self.data.raw_data.len())
  }
}
