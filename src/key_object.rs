use napi::bindgen_prelude::*;
use napi_derive::napi;
use sha2::{Digest, Sha256, Sha512};
use x509_parser::parse_x509_certificate;

#[napi]
pub struct KeyObject {
  pub key_type: String,
  pub asymmetric_key_type: Option<String>,
  pub raw_data: Vec<u8>,
}

#[napi]
impl KeyObject {
  #[napi(constructor)]
  pub fn new(key_type: String, raw_data: Buffer, asymmetric_key_type: Option<String>) -> Self {
    KeyObject {
      key_type,
      asymmetric_key_type,
      raw_data: raw_data.as_ref().to_vec(),
    }
  }

  #[napi(getter)]
  pub fn type_(&self) -> String {
    self.key_type.clone()
  }

  #[napi(getter)]
  pub fn asymmetric_key_type(&self) -> Option<String> {
    self.asymmetric_key_type.clone()
  }

  #[napi]
  pub fn export(&self, format: Option<String>) -> Either<Buffer, String> {
    if format.as_deref() == Some("pem") {
      Either::B(String::from_utf8_lossy(&self.raw_data).to_string())
    } else {
      Either::A(Buffer::from(self.raw_data.clone()))
    }
  }

  #[napi]
  pub fn equals(&self, other: &KeyObject) -> bool {
    self.key_type == other.key_type
      && self.asymmetric_key_type == other.asymmetric_key_type
      && self.raw_data == other.raw_data
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
      public_key_data: public_key.as_ref().to_vec(),
      private_key_data: private_key.as_ref().to_vec(),
    }
  }

  #[napi(getter)]
  pub fn public_key(&self) -> KeyObject {
    KeyObject::new(
      "public".to_string(),
      Buffer::from(self.public_key_data.clone()),
      Some("rsa".to_string()),
    )
  }

  #[napi(getter)]
  pub fn private_key(&self) -> KeyObject {
    KeyObject::new(
      "private".to_string(),
      Buffer::from(self.private_key_data.clone()),
      Some("rsa".to_string()),
    )
  }
}

#[napi]
pub struct X509Certificate {
  raw_bytes: Vec<u8>,
  subject_str: String,
  issuer_str: String,
  valid_from_str: String,
  valid_to_str: String,
  serial_number_str: String,
}

#[napi]
impl X509Certificate {
  #[napi(constructor)]
  pub fn new(input: Either<Buffer, String>) -> Result<Self> {
    let bytes = match input {
      Either::A(b) => b.as_ref().to_vec(),
      Either::B(s) => s.as_bytes().to_vec(),
    };

    let (subject, issuer, valid_from, valid_to, serial) = if bytes.starts_with(b"-----BEGIN CERTIFICATE-----") {
      let pem_res = pem::parse(&bytes)
        .map_err(|e| Error::from_reason(format!("PEM parse error: {e}")))?;
      let (_, cert) = parse_x509_certificate(pem_res.contents())
        .map_err(|e| Error::from_reason(format!("X509 parse error: {e}")))?;
      (
        cert.subject().to_string(),
        cert.issuer().to_string(),
        cert.validity().not_before.to_string(),
        cert.validity().not_after.to_string(),
        cert.raw_serial_as_string(),
      )
    } else {
      let (_, cert) = parse_x509_certificate(&bytes)
        .map_err(|e| Error::from_reason(format!("X509 parse error: {e}")))?;
      (
        cert.subject().to_string(),
        cert.issuer().to_string(),
        cert.validity().not_before.to_string(),
        cert.validity().not_after.to_string(),
        cert.raw_serial_as_string(),
      )
    };

    Ok(X509Certificate {
      raw_bytes: bytes,
      subject_str: subject,
      issuer_str: issuer,
      valid_from_str: valid_from,
      valid_to_str: valid_to,
      serial_number_str: serial,
    })
  }

  #[napi(getter)]
  pub fn raw(&self) -> Buffer {
    Buffer::from(self.raw_bytes.clone())
  }

  #[napi(getter)]
  pub fn subject(&self) -> String {
    self.subject_str.clone()
  }

  #[napi(getter)]
  pub fn issuer(&self) -> String {
    self.issuer_str.clone()
  }

  #[napi(getter)]
  pub fn valid_from(&self) -> String {
    self.valid_from_str.clone()
  }

  #[napi(getter)]
  pub fn valid_to(&self) -> String {
    self.valid_to_str.clone()
  }

  #[napi(getter)]
  pub fn serial_number(&self) -> String {
    self.serial_number_str.clone()
  }

  #[napi(getter)]
  pub fn fingerprint256(&self) -> String {
    let mut hasher = Sha256::new();
    hasher.update(&self.raw_bytes);
    let hash = hasher.finalize();
    hex::encode_upper(hash)
      .as_bytes()
      .chunks(2)
      .map(|chunk| std::str::from_utf8(chunk).unwrap())
      .collect::<Vec<_>>()
      .join(":")
  }

  #[napi(getter)]
  pub fn fingerprint512(&self) -> String {
    let mut hasher = Sha512::new();
    hasher.update(&self.raw_bytes);
    let hash = hasher.finalize();
    hex::encode_upper(hash)
      .as_bytes()
      .chunks(2)
      .map(|chunk| std::str::from_utf8(chunk).unwrap())
      .collect::<Vec<_>>()
      .join(":")
  }

  #[napi]
  pub fn check_email(&self, email: String) -> bool {
    self.subject_str.contains(&email)
  }

  #[napi]
  pub fn check_host(&self, name: String) -> bool {
    self.subject_str.contains(&name)
  }

  #[napi]
  pub fn check_ip(&self, ip: String) -> bool {
    self.subject_str.contains(&ip)
  }

  #[napi]
  pub fn to_string(&self) -> String {
    String::from_utf8_lossy(&self.raw_bytes).to_string()
  }
}
