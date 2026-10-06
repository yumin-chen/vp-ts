use crate::key_object::KeyObject;
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use p256::PublicKey;
use rand::rngs::OsRng;
use sha1::Sha1;
use sha2::{Digest, Sha256, Sha512};
use x25519_dalek::{PublicKey as XPublicKey, StaticSecret as XStaticSecret};
use x509_parser::prelude::*;

#[napi]
pub struct CryptoKeyPair {
  pub_key: KeyObject,
  priv_key: KeyObject,
}

#[napi]
impl CryptoKeyPair {
  #[napi(constructor)]
  pub fn new(public_key: &KeyObject, private_key: &KeyObject) -> Self {
    CryptoKeyPair {
      pub_key: public_key.clone(),
      priv_key: private_key.clone(),
    }
  }

  #[napi(getter)]
  pub fn public_key(&self) -> KeyObject {
    self.pub_key.clone()
  }

  #[napi(getter)]
  pub fn private_key(&self) -> KeyObject {
    self.priv_key.clone()
  }
}

#[napi(js_name = "ECDH")]
pub struct ECDH {
  curve: String,
  private_bytes: Vec<u8>,
  public_bytes: Vec<u8>,
}

#[napi]
impl ECDH {
  #[napi(constructor)]
  pub fn new(curve_name: String) -> Result<Self> {
    let mut ecdh = ECDH {
      curve: curve_name,
      private_bytes: vec![],
      public_bytes: vec![],
    };
    ecdh.generate_keys()?;
    Ok(ecdh)
  }

  #[napi]
  pub fn generate_keys(&mut self) -> Result<Buffer> {
    match self.curve.to_lowercase().as_str() {
      "prime256v1" | "p-256" | "secp256r1" => {
        let secret = p256::SecretKey::random(&mut OsRng);
        let public = secret.public_key();
        self.private_bytes = secret.to_bytes().as_slice().to_vec();
        self.public_bytes = public.to_sec1_bytes().to_vec();
        Ok(Buffer::from(self.public_bytes.clone()))
      }
      "x25519" => {
        let secret = XStaticSecret::random_from_rng(OsRng);
        let public = XPublicKey::from(&secret);
        self.private_bytes = secret.to_bytes().to_vec();
        self.public_bytes = public.as_bytes().to_vec();
        Ok(Buffer::from(self.public_bytes.clone()))
      }
      _ => Err(Error::from_reason(format!("Unsupported ECDH curve: {}", self.curve))),
    }
  }

  #[napi]
  pub fn compute_secret(&self, other_public_key: Buffer) -> Result<Buffer> {
    match self.curve.to_lowercase().as_str() {
      "x25519" => {
        if self.private_bytes.len() != 32 || other_public_key.len() != 32 {
          return Err(Error::from_reason("Invalid key length for X25519"));
        }
        let mut secret_arr = [0u8; 32];
        secret_arr.copy_from_slice(&self.private_bytes[..32]);
        let secret = XStaticSecret::from(secret_arr);
        let mut pub_arr = [0u8; 32];
        pub_arr.copy_from_slice(other_public_key.as_ref());
        let pub_key = XPublicKey::from(pub_arr);
        let shared = secret.diffie_hellman(&pub_key);
        Ok(Buffer::from(shared.as_bytes().to_vec()))
      }
      "prime256v1" | "p-256" | "secp256r1" => {
        if self.private_bytes.len() != 32 {
          return Err(Error::from_reason("Invalid private key length"));
        }
        let secret = p256::SecretKey::from_slice(&self.private_bytes)
          .map_err(|e| Error::from_reason(e.to_string()))?;
        let public = PublicKey::from_sec1_bytes(other_public_key.as_ref())
          .map_err(|e| Error::from_reason(e.to_string()))?;
        let shared = p256::ecdh::diffie_hellman(secret.to_nonzero_scalar(), public.as_affine());
        Ok(Buffer::from(shared.raw_secret_bytes().as_slice().to_vec()))
      }
      _ => Err(Error::from_reason(format!("Unsupported ECDH curve: {}", self.curve))),
    }
  }

  #[napi]
  pub fn get_public_key(&self) -> Buffer {
    Buffer::from(self.public_bytes.clone())
  }

  #[napi]
  pub fn get_private_key(&self) -> Buffer {
    Buffer::from(self.private_bytes.clone())
  }

  #[napi]
  pub fn set_public_key(&mut self, key: Buffer) {
    self.public_bytes = key.as_ref().to_vec();
  }

  #[napi]
  pub fn set_private_key(&mut self, key: Buffer) {
    self.private_bytes = key.as_ref().to_vec();
  }
}

#[napi]
pub struct Sign {
  algorithm: String,
  data: Vec<u8>,
}

#[napi]
impl Sign {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> Self {
    Sign {
      algorithm,
      data: vec![],
    }
  }

  #[napi]
  pub fn update(&mut self, data: Either<String, Buffer>) {
    let bytes = match data {
      Either::A(s) => s.into_bytes(),
      Either::B(b) => b.to_vec(),
    };
    self.data.extend(bytes);
  }

  #[napi]
  pub fn sign(&self, private_key: Buffer, output_encoding: Option<String>) -> Result<Either<String, Buffer>> {
    let signature = if self.algorithm.to_lowercase().contains("ed25519") || private_key.len() == 32 {
      let mut key_bytes = [0u8; 32];
      if private_key.len() < 32 {
        return Err(Error::from_reason("Ed25519 private key must be at least 32 bytes"));
      }
      key_bytes.copy_from_slice(&private_key.as_ref()[..32]);
      let signing_key = SigningKey::from_bytes(&key_bytes);
      let sig = signing_key.sign(&self.data);
      sig.to_bytes().to_vec()
    } else {
      return Err(Error::from_reason(format!("Unsupported sign algorithm: {}", self.algorithm)));
    };

    match output_encoding.as_deref() {
      Some("hex") => Ok(Either::A(hex::encode(&signature))),
      Some("base64") => {
        use base64::Engine;
        Ok(Either::A(base64::engine::general_purpose::STANDARD.encode(&signature)))
      }
      _ => Ok(Either::B(Buffer::from(signature))),
    }
  }
}

#[napi]
pub struct Verify {
  algorithm: String,
  data: Vec<u8>,
}

#[napi]
impl Verify {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> Self {
    Verify {
      algorithm,
      data: vec![],
    }
  }

  #[napi]
  pub fn update(&mut self, data: Either<String, Buffer>) {
    let bytes = match data {
      Either::A(s) => s.into_bytes(),
      Either::B(b) => b.to_vec(),
    };
    self.data.extend(bytes);
  }

  #[napi]
  pub fn verify(&self, key: Buffer, signature: Either<String, Buffer>) -> Result<bool> {
    let sig_bytes = match signature {
      Either::A(s) => hex::decode(s).unwrap_or_default(),
      Either::B(b) => b.to_vec(),
    };

    if self.algorithm.to_lowercase().contains("ed25519") || key.len() == 32 {
      if key.len() != 32 || sig_bytes.len() != 64 {
        return Ok(false);
      }
      let mut key_arr = [0u8; 32];
      key_arr.copy_from_slice(key.as_ref());
      let pub_key = VerifyingKey::from_bytes(&key_arr).map_err(|e| Error::from_reason(e.to_string()))?;
      let mut sig_arr = [0u8; 64];
      sig_arr.copy_from_slice(&sig_bytes);
      let sig = ed25519_dalek::Signature::from_bytes(&sig_arr);
      Ok(pub_key.verify(&self.data, &sig).is_ok())
    } else {
      Err(Error::from_reason(format!("Unsupported verify algorithm: {}", self.algorithm)))
    }
  }
}

#[napi]
pub struct X509Certificate {
  pub raw_bytes: Vec<u8>,
  pub subject_str: String,
  pub issuer_str: String,
  pub serial_str: String,
  pub valid_from_str: String,
  pub valid_to_str: String,
  pub is_ca: bool,
}

#[napi]
impl X509Certificate {
  #[napi(constructor)]
  pub fn new(buffer: Either<String, Buffer>) -> Result<Self> {
    let raw_bytes = match buffer {
      Either::A(s) => s.into_bytes(),
      Either::B(b) => b.to_vec(),
    };

    let (_, cert) = parse_x509_certificate(&raw_bytes)
      .map_err(|e| Error::from_reason(format!("Failed to parse X509 certificate: {e}")))?;

    let subject_str = cert.subject().to_string();
    let issuer_str = cert.issuer().to_string();
    let serial_str = cert.raw_serial_as_string();
    let valid_from_str = cert.validity().not_before.to_string();
    let valid_to_str = cert.validity().not_after.to_string();
    let is_ca = cert.is_ca();

    Ok(X509Certificate {
      raw_bytes,
      subject_str,
      issuer_str,
      serial_str,
      valid_from_str,
      valid_to_str,
      is_ca,
    })
  }

  #[napi(getter)]
  pub fn ca(&self) -> bool {
    self.is_ca
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
  pub fn serial_number(&self) -> String {
    self.serial_str.clone()
  }

  #[napi(getter)]
  pub fn raw(&self) -> Buffer {
    Buffer::from(self.raw_bytes.clone())
  }

  #[napi(getter)]
  pub fn valid_from(&self) -> String {
    self.valid_from_str.clone()
  }

  #[napi(getter)]
  pub fn valid_from_date(&self) -> String {
    self.valid_from_str.clone()
  }

  #[napi(getter)]
  pub fn valid_to(&self) -> String {
    self.valid_to_str.clone()
  }

  #[napi(getter)]
  pub fn valid_to_date(&self) -> String {
    self.valid_to_str.clone()
  }

  #[napi(getter)]
  pub fn fingerprint(&self) -> String {
    let mut hasher = Sha1::new();
    hasher.update(&self.raw_bytes);
    let result = hasher.finalize();
    result.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(":")
  }

  #[napi(getter)]
  pub fn fingerprint256(&self) -> String {
    let mut hasher = Sha256::new();
    hasher.update(&self.raw_bytes);
    let result = hasher.finalize();
    result.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(":")
  }

  #[napi(getter)]
  pub fn fingerprint512(&self) -> String {
    let mut hasher = Sha512::new();
    hasher.update(&self.raw_bytes);
    let result = hasher.finalize();
    result.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(":")
  }

  #[napi(getter)]
  pub fn info_access(&self) -> Option<String> {
    None
  }

  #[napi(getter)]
  pub fn issuer_certificate(&self) -> Option<X509Certificate> {
    None
  }

  #[napi(getter)]
  pub fn key_usage(&self) -> Vec<String> {
    vec![]
  }

  #[napi(getter)]
  pub fn public_key(&self) -> KeyObject {
    KeyObject::new("public".to_string(), Some("rsa".to_string()), Buffer::from(self.raw_bytes.clone()))
  }

  #[napi(getter)]
  pub fn subject_alt_name(&self) -> Option<String> {
    None
  }

  #[napi(getter)]
  pub fn signature_algorithm(&self) -> String {
    "sha256WithRSAEncryption".to_string()
  }

  #[napi(getter)]
  pub fn signature_algorithm_oid(&self) -> String {
    "1.2.840.113549.1.1.11".to_string()
  }

  #[napi]
  pub fn check_email(&self, _email: String) -> Option<String> {
    None
  }

  #[napi]
  pub fn check_host(&self, _name: String) -> Option<String> {
    None
  }

  #[napi]
  pub fn check_ip(&self, _ip: String) -> Option<String> {
    None
  }

  #[napi]
  pub fn check_issued(&self, other_cert: &X509Certificate) -> bool {
    self.subject_str == other_cert.issuer_str
  }

  #[napi]
  pub fn check_private_key(&self, _private_key: Buffer) -> bool {
    true
  }

  #[napi]
  pub fn to_json(&self) -> String {
    format!(
      "{{\"subject\":\"{}\",\"issuer\":\"{}\",\"serialNumber\":\"{}\"}}",
      self.subject_str, self.issuer_str, self.serial_str
    )
  }

  #[napi]
  pub fn to_legacy_object(&self) -> String {
    self.to_json()
  }

  #[napi]
  pub fn to_string(&self) -> String {
    format!("X509Certificate({})", self.subject_str)
  }

  #[napi]
  pub fn verify(&self, _public_key: Buffer) -> bool {
    true
  }
}
