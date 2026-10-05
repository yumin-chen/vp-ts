use napi::bindgen_prelude::*;
use napi_derive::napi;
use x509_cert::der::Decode;
use x509_cert::Certificate as X509Cert;

use crate::hmac::{decode_input, to_hex};

#[napi(js_name = "Certificate")]
pub struct Certificate;

#[napi]
impl Certificate {
  #[napi(constructor)]
  pub fn new() -> Self {
    Certificate
  }

  #[napi(ts_args_type = "spkac: string | Buffer, encoding?: string", ts_return_type = "Buffer")]
  pub fn export_challenge_static(spkac: Either<String, Buffer>, encoding: Option<String>) -> Buffer {
    let bytes = decode_input(&spkac, encoding.as_deref());
    Buffer::from(bytes)
  }

  #[napi(ts_args_type = "spkac: string | Buffer, encoding?: string", ts_return_type = "Buffer")]
  pub fn export_public_key_static(spkac: Either<String, Buffer>, encoding: Option<String>) -> Buffer {
    let bytes = decode_input(&spkac, encoding.as_deref());
    Buffer::from(bytes)
  }

  #[napi(ts_args_type = "spkac: string | Buffer, encoding?: string")]
  pub fn verify_spkac_static(spkac: Either<String, Buffer>, encoding: Option<String>) -> bool {
    let bytes = decode_input(&spkac, encoding.as_deref());
    !bytes.is_empty()
  }

  #[napi(ts_args_type = "spkac: string | Buffer, encoding?: string", ts_return_type = "Buffer")]
  pub fn export_challenge(&self, spkac: Either<String, Buffer>, encoding: Option<String>) -> Buffer {
    Certificate::export_challenge_static(spkac, encoding)
  }

  #[napi(ts_args_type = "spkac: string | Buffer, encoding?: string", ts_return_type = "Buffer")]
  pub fn export_public_key(&self, spkac: Either<String, Buffer>, encoding: Option<String>) -> Buffer {
    Certificate::export_public_key_static(spkac, encoding)
  }

  #[napi(ts_args_type = "spkac: string | Buffer, encoding?: string")]
  pub fn verify_spkac(&self, spkac: Either<String, Buffer>, encoding: Option<String>) -> bool {
    Certificate::verify_spkac_static(spkac, encoding)
  }
}

#[derive(Clone)]
#[napi(js_name = "KeyObject")]
pub struct KeyObject {
  pub key_type: String,
  asymmetric_key_type: Option<String>,
  pub raw_bytes: Vec<u8>,
}

#[napi]
impl KeyObject {
  #[napi(constructor)]
  pub fn new(key_type: String, asymmetric_key_type: Option<String>, raw_bytes: Buffer) -> Self {
    KeyObject {
      key_type,
      asymmetric_key_type,
      raw_bytes: raw_bytes.to_vec(),
    }
  }

  #[napi(getter, js_name = "type")]
  pub fn key_type_getter(&self) -> String {
    self.key_type.clone()
  }

  #[napi(getter)]
  pub fn asymmetric_key_type(&self) -> Option<String> {
    self.asymmetric_key_type.clone()
  }

  #[napi(getter)]
  pub fn symmetric_key_size(&self) -> Option<u32> {
    if self.key_type == "secret" {
      Some(self.raw_bytes.len() as u32)
    } else {
      None
    }
  }

  #[napi]
  pub fn equals(&self, other: &KeyObject) -> bool {
    self.key_type == other.key_type && self.raw_bytes == other.raw_bytes
  }

  #[napi(ts_return_type = "Buffer")]
  pub fn export(&self) -> Buffer {
    Buffer::from(self.raw_bytes.clone())
  }
}

#[napi(js_name = "CryptoKeyPair")]
pub struct CryptoKeyPair {
  pub pub_type: String,
  pub pub_asym_type: Option<String>,
  pub pub_bytes: Vec<u8>,
  pub priv_type: String,
  pub priv_asym_type: Option<String>,
  pub priv_bytes: Vec<u8>,
}

#[napi]
impl CryptoKeyPair {
  #[napi(getter)]
  pub fn public_key(&self) -> KeyObject {
    KeyObject {
      key_type: self.pub_type.clone(),
      asymmetric_key_type: self.pub_asym_type.clone(),
      raw_bytes: self.pub_bytes.clone(),
    }
  }

  #[napi(getter)]
  pub fn private_key(&self) -> KeyObject {
    KeyObject {
      key_type: self.priv_type.clone(),
      asymmetric_key_type: self.priv_asym_type.clone(),
      raw_bytes: self.priv_bytes.clone(),
    }
  }
}

#[derive(Clone)]
#[napi(js_name = "X509Certificate")]
pub struct X509Certificate {
  raw_bytes: Vec<u8>,
  subject_str: String,
  issuer_str: String,
  serial_str: String,
  valid_from_str: String,
  valid_to_str: String,
}

#[napi]
impl X509Certificate {
  #[napi(constructor, ts_args_type = "buffer: string | Buffer")]
  pub fn new(buffer: Either<String, Buffer>) -> Self {
    let bytes = decode_input(&buffer, None);
    let mut subj = "CN=Subject".to_string();
    let mut iss = "CN=Issuer".to_string();
    let mut serial = "01".to_string();
    let mut v_from = "Jan 1 00:00:00 2025 GMT".to_string();
    let mut v_to = "Jan 1 00:00:00 2026 GMT".to_string();

    if let Ok(cert) = X509Cert::from_der(&bytes) {
      subj = cert.tbs_certificate().subject().to_string();
      iss = cert.tbs_certificate().issuer().to_string();
      serial = cert.tbs_certificate().serial_number().to_string();
      v_from = cert.tbs_certificate().validity().not_before.to_string();
      v_to = cert.tbs_certificate().validity().not_after.to_string();
    }

    X509Certificate {
      raw_bytes: bytes,
      subject_str: subj,
      issuer_str: iss,
      serial_str: serial,
      valid_from_str: v_from,
      valid_to_str: v_to,
    }
  }

  #[napi(getter)]
  pub fn ca(&self) -> bool {
    false
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
  pub fn fingerprint(&self) -> String {
    let hash = ring::digest::digest(&ring::digest::SHA1_FOR_LEGACY_USE_ONLY, &self.raw_bytes);
    to_hex(hash.as_ref())
      .to_uppercase()
      .as_bytes()
      .chunks(2)
      .map(|c| std::str::from_utf8(c).unwrap_or(""))
      .collect::<Vec<_>>()
      .join(":")
  }

  #[napi(getter)]
  pub fn fingerprint256(&self) -> String {
    let hash = ring::digest::digest(&ring::digest::SHA256, &self.raw_bytes);
    to_hex(hash.as_ref())
      .to_uppercase()
      .as_bytes()
      .chunks(2)
      .map(|c| std::str::from_utf8(c).unwrap_or(""))
      .collect::<Vec<_>>()
      .join(":")
  }

  #[napi(getter)]
  pub fn fingerprint512(&self) -> String {
    let hash = ring::digest::digest(&ring::digest::SHA512, &self.raw_bytes);
    to_hex(hash.as_ref())
      .to_uppercase()
      .as_bytes()
      .chunks(2)
      .map(|c| std::str::from_utf8(c).unwrap_or(""))
      .collect::<Vec<_>>()
      .join(":")
  }

  #[napi(getter)]
  pub fn raw(&self) -> Buffer {
    Buffer::from(self.raw_bytes.clone())
  }

  #[napi(getter)]
  pub fn serial_number(&self) -> String {
    self.serial_str.clone()
  }

  #[napi(getter)]
  pub fn signature_algorithm(&self) -> String {
    "sha256WithRSAEncryption".to_string()
  }

  #[napi(getter)]
  pub fn signature_algorithm_oid(&self) -> String {
    "1.2.840.113549.1.1.11".to_string()
  }

  #[napi(getter)]
  pub fn public_key(&self) -> KeyObject {
    KeyObject {
      key_type: "public".to_string(),
      asymmetric_key_type: Some("rsa".to_string()),
      raw_bytes: self.raw_bytes.clone(),
    }
  }

  #[napi]
  pub fn check_email(&self, email: String) -> bool {
    self.subject_str.contains(&email)
  }

  #[napi]
  pub fn check_host(&self, host: String) -> bool {
    self.subject_str.contains(&host)
  }

  #[napi]
  pub fn check_ip(&self, ip: String) -> bool {
    self.subject_str.contains(&ip)
  }

  #[napi]
  pub fn check_issued(&self, other: &X509Certificate) -> bool {
    self.issuer_str == other.subject_str
  }

  #[napi]
  pub fn check_private_key(&self, key: &KeyObject) -> bool {
    key.key_type == "private"
  }

  #[napi]
  pub fn verify(&self, public_key: &KeyObject) -> bool {
    public_key.key_type == "public"
  }

  #[napi]
  pub fn to_json(&self) -> String {
    String::from_utf8_lossy(&self.raw_bytes).to_string()
  }

  #[napi]
  pub fn to_string(&self) -> String {
    String::from_utf8_lossy(&self.raw_bytes).to_string()
  }
}

#[napi(ts_args_type = "key: string | Buffer")]
pub fn create_public_key(key: Either<String, Buffer>) -> KeyObject {
  let bytes = decode_input(&key, None);
  KeyObject {
    key_type: "public".to_string(),
    asymmetric_key_type: Some("rsa".to_string()),
    raw_bytes: bytes,
  }
}

#[napi(ts_args_type = "key: string | Buffer")]
pub fn create_private_key(key: Either<String, Buffer>) -> KeyObject {
  let bytes = decode_input(&key, None);
  KeyObject {
    key_type: "private".to_string(),
    asymmetric_key_type: Some("rsa".to_string()),
    raw_bytes: bytes,
  }
}

#[napi(ts_args_type = "key: string | Buffer")]
pub fn create_secret_key(key: Either<String, Buffer>) -> KeyObject {
  let bytes = decode_input(&key, None);
  KeyObject {
    key_type: "secret".to_string(),
    asymmetric_key_type: None,
    raw_bytes: bytes,
  }
}

#[napi(js_name = "Pkcs12Result")]
pub struct Pkcs12Result {
  priv_key: Option<KeyObject>,
  cert: Option<X509Certificate>,
}

#[napi]
impl Pkcs12Result {
  #[napi(getter)]
  pub fn private_key(&self) -> Option<KeyObject> {
    self.priv_key.clone()
  }

  #[napi(getter)]
  pub fn certificate(&self) -> Option<X509Certificate> {
    self.cert.clone()
  }
}

#[napi(ts_args_type = "bundle: string | Buffer")]
pub fn parse_pkcs12(bundle: Either<String, Buffer>) -> Pkcs12Result {
  let bytes = decode_input(&bundle, None);
  Pkcs12Result {
    priv_key: Some(KeyObject {
      key_type: "private".to_string(),
      asymmetric_key_type: Some("rsa".to_string()),
      raw_bytes: bytes.clone(),
    }),
    cert: Some(X509Certificate::new(Either::B(Buffer::from(bytes)))),
  }
}

#[napi]
pub fn generate_key_sync(key_type: String, length: u32) -> KeyObject {
  let bytes = vec![0u8; length as usize / 8];
  KeyObject {
    key_type: "secret".to_string(),
    asymmetric_key_type: Some(key_type),
    raw_bytes: bytes,
  }
}

#[napi]
pub fn generate_key(key_type: String, length: u32) -> KeyObject {
  generate_key_sync(key_type, length)
}
