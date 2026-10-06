use napi::bindgen_prelude::*;
use napi_derive::napi;
use p256::elliptic_curve::ecdh::diffie_hellman;
use p256::elliptic_curve::sec1::ToSec1Point;
use p256::SecretKey as P256SecretKey;
use p256::PublicKey as P256PublicKey;
use p384::SecretKey as P384SecretKey;
use p384::PublicKey as P384PublicKey;
use ring::rand::{SystemRandom, SecureRandom};

fn random_bytes(len: usize) -> Result<Vec<u8>> {
  let rng = SystemRandom::new();
  let mut bytes = vec![0u8; len];
  rng.fill(&mut bytes)
    .map_err(|_| Error::new(Status::GenericFailure, "Random generation failed"))?;
  Ok(bytes)
}

#[napi]
pub struct ECDH {
  curve_name: String,
  private_key: Option<Vec<u8>>,
  public_key: Option<Vec<u8>>,
}

#[napi]
impl ECDH {
  #[napi(constructor)]
  pub fn new(curve_name: String) -> Result<Self> {
    let clean = curve_name.to_lowercase();
    match clean.as_str() {
      "prime256v1" | "secp256r1" | "p-256" | "secp384r1" | "p-384" => Ok(Self {
        curve_name: clean,
        private_key: None,
        public_key: None,
      }),
      _ => Err(Error::new(
        Status::InvalidArg,
        format!("Unknown or unsupported curve: {}", curve_name),
      )),
    }
  }

  #[napi]
  pub fn generate_keys(&mut self) -> Result<Buffer> {
    match self.curve_name.as_str() {
      "prime256v1" | "secp256r1" | "p-256" => {
        let bytes = random_bytes(32)?;
        let sk = P256SecretKey::from_slice(&bytes)
          .map_err(|e| Error::new(Status::GenericFailure, format!("Key generation error: {}", e)))?;
        let pk = sk.public_key();
        self.private_key = Some(sk.to_bytes().to_vec());
        self.public_key = Some(pk.to_sec1_point(false).as_bytes().to_vec());
        Ok(Buffer::from(self.public_key.clone().unwrap()))
      }
      "secp384r1" | "p-384" => {
        let bytes = random_bytes(48)?;
        let sk = P384SecretKey::from_slice(&bytes)
          .map_err(|e| Error::new(Status::GenericFailure, format!("Key generation error: {}", e)))?;
        let pk = sk.public_key();
        self.private_key = Some(sk.to_bytes().to_vec());
        self.public_key = Some(pk.to_sec1_point(false).as_bytes().to_vec());
        Ok(Buffer::from(self.public_key.clone().unwrap()))
      }
      _ => Err(Error::new(Status::InvalidArg, "Unsupported curve")),
    }
  }

  #[napi]
  pub fn compute_secret(&self, other_public_key: Buffer) -> Result<Buffer> {
    let priv_bytes = self
      .private_key
      .as_ref()
      .ok_or_else(|| Error::new(Status::GenericFailure, "Private key not generated"))?;

    match self.curve_name.as_str() {
      "prime256v1" | "secp256r1" | "p-256" => {
        let sk = P256SecretKey::from_slice(priv_bytes)
          .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid private key: {}", e)))?;
        let pk = P256PublicKey::from_sec1_bytes(&other_public_key)
          .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid public key: {}", e)))?;
        let secret = diffie_hellman(sk.to_nonzero_scalar(), pk.as_affine());
        Ok(Buffer::from(secret.raw_secret_bytes().as_slice()))
      }
      "secp384r1" | "p-384" => {
        let sk = P384SecretKey::from_slice(priv_bytes)
          .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid private key: {}", e)))?;
        let pk = P384PublicKey::from_sec1_bytes(&other_public_key)
          .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid public key: {}", e)))?;
        let secret = diffie_hellman(sk.to_nonzero_scalar(), pk.as_affine());
        Ok(Buffer::from(secret.raw_secret_bytes().as_slice()))
      }
      _ => Err(Error::new(Status::InvalidArg, "Unsupported curve")),
    }
  }

  #[napi]
  pub fn get_public_key(&self) -> Result<Buffer> {
    let pk = self
      .public_key
      .as_ref()
      .ok_or_else(|| Error::new(Status::GenericFailure, "Public key not available"))?;
    Ok(Buffer::from(pk.clone()))
  }

  #[napi]
  pub fn get_private_key(&self) -> Result<Buffer> {
    let sk = self
      .private_key
      .as_ref()
      .ok_or_else(|| Error::new(Status::GenericFailure, "Private key not available"))?;
    Ok(Buffer::from(sk.clone()))
  }
}

#[napi]
pub fn create_ecdh(curve_name: String) -> Result<ECDH> {
  ECDH::new(curve_name)
}

#[napi]
pub fn get_curves() -> Vec<String> {
  vec![
    "prime256v1".to_string(),
    "secp256r1".to_string(),
    "p-256".to_string(),
    "secp384r1".to_string(),
    "p-384".to_string(),
  ]
}
