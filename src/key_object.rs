use napi::bindgen_prelude::*;
use napi_derive::napi;
use x509_parser::prelude::parse_x509_certificate;

#[napi]
pub struct KeyObject {
  key_type: String, // "secret", "public", "private"
  asymmetric_key_type: Option<String>,
  raw_data: Vec<u8>,
}

#[napi]
impl KeyObject {
  #[napi(constructor)]
  pub fn new(key_type: String, raw_data: Buffer, asymmetric_key_type: Option<String>) -> Self {
    Self {
      key_type,
      asymmetric_key_type,
      raw_data: raw_data.to_vec(),
    }
  }

  #[napi(getter)]
  pub fn get_type(&self) -> String {
    self.key_type.clone()
  }

  #[napi(getter)]
  pub fn get_asymmetric_key_type(&self) -> Option<String> {
    self.asymmetric_key_type.clone()
  }

  #[napi]
  pub fn export(&self) -> Buffer {
    Buffer::from(self.raw_data.clone())
  }
}

#[napi(object)]
pub struct CryptoKeyPair {
  pub public_key: String,
  pub private_key: String,
}

#[napi]
pub struct X509Certificate {
  raw: Vec<u8>,
  subject: String,
  issuer: String,
  valid_from: String,
  valid_to: String,
  serial_number: String,
  fingerprint256: String,
}

#[napi]
impl X509Certificate {
  #[napi(constructor)]
  pub fn new(buffer: Buffer) -> Result<Self> {
    let (_, cert) = parse_x509_certificate(&buffer)
      .map_err(|e| Error::new(Status::InvalidArg, format!("Failed to parse X509 certificate: {}", e)))?;

    let subject = cert.subject().to_string();
    let issuer = cert.issuer().to_string();
    let valid_from = cert.validity().not_before.to_string();
    let valid_to = cert.validity().not_after.to_string();
    let serial_number = cert.raw_serial_as_string();

    use sha2::{Digest, Sha256};
    let hash = Sha256::digest(&buffer);
    let fingerprint256 = hex::encode(hash);

    Ok(Self {
      raw: buffer.to_vec(),
      subject,
      issuer,
      valid_from,
      valid_to,
      serial_number,
      fingerprint256,
    })
  }

  #[napi(getter)]
  pub fn get_subject(&self) -> String {
    self.subject.clone()
  }

  #[napi(getter)]
  pub fn get_issuer(&self) -> String {
    self.issuer.clone()
  }

  #[napi(getter)]
  pub fn get_valid_from(&self) -> String {
    self.valid_from.clone()
  }

  #[napi(getter)]
  pub fn get_valid_to(&self) -> String {
    self.valid_to.clone()
  }

  #[napi(getter)]
  pub fn get_serial_number(&self) -> String {
    self.serial_number.clone()
  }

  #[napi(getter)]
  pub fn get_fingerprint256(&self) -> String {
    self.fingerprint256.clone()
  }

  #[napi(getter)]
  pub fn get_raw(&self) -> Buffer {
    Buffer::from(self.raw.clone())
  }

  #[napi]
  pub fn to_string(&self) -> String {
    format!(
      "Subject: {}\nIssuer: {}\nValid From: {}\nValid To: {}\nSerial: {}",
      self.subject, self.issuer, self.valid_from, self.valid_to, self.serial_number
    )
  }
}
