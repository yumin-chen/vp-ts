use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::digest;
use x509_parser::certificate::X509Certificate as X509ParserCert;
use x509_parser::prelude::FromDer;

#[napi]
pub enum KeyObjectType {
  Secret,
  Public,
  Private,
}

#[napi]
pub struct KeyObject {
  key_type: KeyObjectType,
  data: Vec<u8>,
  asymmetric_type: Option<String>,
}

#[napi]
impl KeyObject {
  #[napi(constructor)]
  pub fn new(
    key_type: KeyObjectType,
    data: Either<String, Uint8Array>,
    asymmetric_type: Option<String>,
  ) -> Self {
    let bytes = match data {
      Either::A(s) => s.into_bytes(),
      Either::B(b) => b.as_ref().to_vec(),
    };
    Self {
      key_type,
      data: bytes,
      asymmetric_type,
    }
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
  pub fn get_asymmetric_key_type(&self) -> Option<String> {
    self.asymmetric_type.clone()
  }

  #[napi]
  pub fn export(&self) -> Buffer {
    Buffer::from(self.data.clone())
  }
}

#[napi]
pub fn create_secret_key(key: Either<String, Uint8Array>) -> KeyObject {
  KeyObject::new(KeyObjectType::Secret, key, None)
}

#[napi]
pub fn create_public_key(
  key: Either<String, Uint8Array>,
  asymmetric_type: Option<String>,
) -> KeyObject {
  KeyObject::new(KeyObjectType::Public, key, asymmetric_type)
}

#[napi]
pub fn create_private_key(
  key: Either<String, Uint8Array>,
  asymmetric_type: Option<String>,
) -> KeyObject {
  KeyObject::new(KeyObjectType::Private, key, asymmetric_type)
}

#[napi]
#[allow(dead_code)]
pub struct X509Certificate {
  raw: Vec<u8>,
  subject: String,
  issuer: String,
  valid_from: String,
  valid_to: String,
  serial_number: String,
  fingerprint_256: String,
}

#[napi]
impl X509Certificate {
  #[napi(constructor)]
  pub fn new(buffer: Uint8Array) -> Result<Self> {
    let bytes = buffer.as_ref();
    let (_, cert) = X509ParserCert::from_der(bytes)
      .map_err(|e| Error::new(Status::InvalidArg, format!("Failed to parse X509 cert: {}", e)))?;

    let subject = cert.subject().to_string();
    let issuer = cert.issuer().to_string();
    let valid_from = cert.validity().not_before.to_string();
    let valid_to = cert.validity().not_after.to_string();
    let serial = cert.raw_serial_as_string();

    let digest_256 = digest::digest(&digest::SHA256, bytes);
    let fingerprint_256 = hex::encode(digest_256.as_ref());

    Ok(Self {
      raw: bytes.to_vec(),
      subject,
      issuer,
      valid_from,
      valid_to,
      serial_number: serial,
      fingerprint_256,
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
    self.fingerprint_256.clone()
  }

  #[napi]
  pub fn to_string(&self) -> String {
    format!(
      "X509Certificate [subject: {}, issuer: {}, serial: {}]",
      self.subject, self.issuer, self.serial_number
    )
  }
}
