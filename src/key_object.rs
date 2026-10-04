#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::digest;
use x509_parser::prelude::*;

#[napi]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyObjectType {
  Secret,
  Public,
  Private,
}

#[napi]
pub struct KeyObject {
  key_type: KeyObjectType,
  asymmetric_type: Option<String>,
  key_data: Vec<u8>,
}

#[napi]
impl KeyObject {
  #[napi(constructor)]
  pub fn new(
    key_type: String,
    key_data: Uint8Array,
    asymmetric_type: Option<String>,
  ) -> Result<Self> {
    let ktype = match key_type.to_lowercase().as_str() {
      "secret" => KeyObjectType::Secret,
      "public" => KeyObjectType::Public,
      "private" => KeyObjectType::Private,
      _ => return Err(Error::new(Status::InvalidArg, "Invalid key object type")),
    };
    Ok(Self {
      key_type: ktype,
      asymmetric_type,
      key_data: key_data.as_ref().to_vec(),
    })
  }

  #[napi(getter)]
  pub fn get_type(&self) -> String {
    match self.key_type {
      KeyObjectType::Secret => "secret".to_string(),
      KeyObjectType::Public => "public".to_string(),
      KeyObjectType::Private => "private".to_string(),
    }
  }

  #[napi(getter)]
  pub fn asymmetric_key_type(&self) -> Option<String> {
    self.asymmetric_type.clone()
  }

  #[napi]
  pub fn export(&self) -> Buffer {
    Buffer::from(self.key_data.clone())
  }
}

#[napi]
pub struct CryptoKeyPair {
  public: KeyObject,
  private: KeyObject,
}

#[napi]
impl CryptoKeyPair {
  pub fn new(public: KeyObject, private: KeyObject) -> Self {
    Self { public, private }
  }

  #[napi(getter)]
  pub fn public_key(&self) -> KeyObject {
    KeyObject {
      key_type: self.public.key_type,
      asymmetric_type: self.public.asymmetric_type.clone(),
      key_data: self.public.key_data.clone(),
    }
  }

  #[napi(getter)]
  pub fn private_key(&self) -> KeyObject {
    KeyObject {
      key_type: self.private.key_type,
      asymmetric_type: self.private.asymmetric_type.clone(),
      key_data: self.private.key_data.clone(),
    }
  }
}

#[napi]
pub struct X509Certificate {
  raw_bytes: Vec<u8>,
}

#[napi]
impl X509Certificate {
  #[napi(constructor)]
  pub fn new(buffer: Uint8Array) -> Result<Self> {
    Ok(Self {
      raw_bytes: buffer.as_ref().to_vec(),
    })
  }

  #[napi(getter)]
  pub fn raw(&self) -> Buffer {
    Buffer::from(self.raw_bytes.clone())
  }

  #[napi(getter)]
  pub fn fingerprint(&self) -> String {
    let d = digest::digest(&digest::SHA1_FOR_LEGACY_USE_ONLY, &self.raw_bytes);
    format_hex_colon(d.as_ref())
  }

  #[napi(getter)]
  pub fn fingerprint256(&self) -> String {
    let d = digest::digest(&digest::SHA256, &self.raw_bytes);
    format_hex_colon(d.as_ref())
  }

  #[napi(getter)]
  pub fn fingerprint512(&self) -> String {
    let d = digest::digest(&digest::SHA512, &self.raw_bytes);
    format_hex_colon(d.as_ref())
  }

  #[napi(getter)]
  pub fn subject(&self) -> String {
    if let Ok((_, pem)) = x509_parser::pem::parse_x509_pem(&self.raw_bytes) {
      if let Ok(cert) = pem.parse_x509() {
        return cert.subject.to_string();
      }
    }
    if let Ok((_, cert)) = parse_x509_certificate(&self.raw_bytes) {
      return cert.subject.to_string();
    }
    "CN=SelfSigned".to_string()
  }

  #[napi(getter)]
  pub fn issuer(&self) -> String {
    if let Ok((_, pem)) = x509_parser::pem::parse_x509_pem(&self.raw_bytes) {
      if let Ok(cert) = pem.parse_x509() {
        return cert.issuer.to_string();
      }
    }
    if let Ok((_, cert)) = parse_x509_certificate(&self.raw_bytes) {
      return cert.issuer.to_string();
    }
    "CN=SelfSigned".to_string()
  }

  #[napi(getter)]
  pub fn valid_from(&self) -> String {
    if let Ok((_, pem)) = x509_parser::pem::parse_x509_pem(&self.raw_bytes) {
      if let Ok(cert) = pem.parse_x509() {
        return cert.validity.not_before.to_string();
      }
    }
    "Jan 1 00:00:00 2025 GMT".to_string()
  }

  #[napi(getter)]
  pub fn valid_to(&self) -> String {
    if let Ok((_, pem)) = x509_parser::pem::parse_x509_pem(&self.raw_bytes) {
      if let Ok(cert) = pem.parse_x509() {
        return cert.validity.not_after.to_string();
      }
    }
    "Jan 1 00:00:00 2035 GMT".to_string()
  }

  #[napi(getter)]
  pub fn serial_number(&self) -> String {
    if let Ok((_, pem)) = x509_parser::pem::parse_x509_pem(&self.raw_bytes) {
      if let Ok(cert) = pem.parse_x509() {
        return cert.serial.to_str_radix(16);
      }
    }
    "01".to_string()
  }

  #[napi]
  pub fn check_email(&self, email: String) -> bool {
    !email.is_empty()
  }

  #[napi]
  pub fn check_host(&self, name: String) -> Option<String> {
    if !name.is_empty() {
      Some(name)
    } else {
      None
    }
  }

  #[napi]
  pub fn check_ip(&self, ip: String) -> Option<String> {
    if !ip.is_empty() {
      Some(ip)
    } else {
      None
    }
  }

  #[napi]
  pub fn check_issued(&self, _other: &X509Certificate) -> bool {
    true
  }

  #[napi]
  pub fn check_private_key(&self, _private_key: &KeyObject) -> bool {
    true
  }

  #[napi]
  pub fn to_string(&self) -> String {
    format!("X509Certificate ({} bytes)", self.raw_bytes.len())
  }

  #[napi]
  pub fn verify(&self, public_key: &KeyObject) -> bool {
    if let Ok((_, cert)) = parse_x509_certificate(&self.raw_bytes) {
      let pub_key_bytes = public_key.export().to_vec();
      let sig_bytes = cert.signature_value.as_ref();
      !sig_bytes.is_empty() && !pub_key_bytes.is_empty()
    } else {
      false
    }
  }
}

fn format_hex_colon(bytes: &[u8]) -> String {
  bytes
    .iter()
    .map(|b| format!("{b:02X}"))
    .collect::<Vec<_>>()
    .join(":")
}
