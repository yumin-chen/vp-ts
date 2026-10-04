use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub enum KeyType {
  Secret,
  Public,
  Private,
}

#[napi]
pub struct KeyObject {
  pub key_type: KeyType,
  pub data: Vec<u8>,
  pub asymmetric_key_type: Option<String>,
}

#[napi]
impl KeyObject {
  #[napi(constructor)]
  pub fn new(key_type: KeyType, data: Buffer, asymmetric_key_type: Option<String>) -> Self {
    KeyObject {
      key_type,
      data: data.to_vec(),
      asymmetric_key_type,
    }
  }

  #[napi]
  pub fn export(&self) -> Buffer {
    Buffer::from(self.data.clone())
  }
}

#[napi]
pub struct CryptoKeyPair {
  public_key_data: Vec<u8>,
  private_key_data: Vec<u8>,
}

#[napi]
impl CryptoKeyPair {
  #[napi(constructor)]
  pub fn new(public_key: Buffer, private_key: Buffer) -> Self {
    CryptoKeyPair {
      public_key_data: public_key.to_vec(),
      private_key_data: private_key.to_vec(),
    }
  }

  #[napi(getter)]
  pub fn public_key(&self) -> KeyObject {
    KeyObject::new(
      KeyType::Public,
      Buffer::from(self.public_key_data.clone()),
      None,
    )
  }

  #[napi(getter)]
  pub fn private_key(&self) -> KeyObject {
    KeyObject::new(
      KeyType::Private,
      Buffer::from(self.private_key_data.clone()),
      None,
    )
  }
}

#[napi]
pub struct X509Certificate {
  pub raw: Vec<u8>,
}

#[napi]
impl X509Certificate {
  #[napi(constructor)]
  pub fn new(buffer: Buffer) -> Self {
    X509Certificate {
      raw: buffer.to_vec(),
    }
  }

  #[napi]
  pub fn to_string(&self) -> String {
    format!("X509Certificate(bytes={})", self.raw.len())
  }
}

#[napi]
pub fn create_secret_key(key: Buffer) -> KeyObject {
  KeyObject::new(KeyType::Secret, key, None)
}

#[napi]
pub fn create_public_key(key: Buffer) -> KeyObject {
  KeyObject::new(KeyType::Public, key, None)
}

#[napi]
pub fn create_private_key(key: Buffer) -> KeyObject {
  KeyObject::new(KeyType::Private, key, None)
}
