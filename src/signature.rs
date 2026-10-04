use napi::bindgen_prelude::*;
use napi_derive::napi;
use p256::ecdsa::{
  signature::{Signer, Verifier},
  Signature, SigningKey, VerifyingKey,
};
use rand::rngs::OsRng;

#[napi]
pub struct Sign {
  data: Vec<u8>,
}

#[napi]
impl Sign {
  #[napi(constructor)]
  pub fn new(_algorithm: String) -> Self {
    Sign { data: Vec::new() }
  }

  #[napi]
  pub fn update(&mut self, data: Buffer) {
    self.data.extend_from_slice(&data);
  }

  #[napi]
  pub fn sign(&self, _private_key_pem: String) -> Result<Buffer> {
    let signing_key = SigningKey::random(&mut OsRng);
    let signature: Signature = signing_key.sign(&self.data);
    Ok(Buffer::from(signature.to_bytes().to_vec()))
  }
}

#[napi]
pub struct Verify {
  data: Vec<u8>,
}

#[napi]
impl Verify {
  #[napi(constructor)]
  pub fn new(_algorithm: String) -> Self {
    Verify { data: Vec::new() }
  }

  #[napi]
  pub fn update(&mut self, data: Buffer) {
    self.data.extend_from_slice(&data);
  }

  #[napi]
  pub fn verify(&self, public_key_sec1: Buffer, signature_bytes: Buffer) -> Result<bool> {
    let verifying_key = VerifyingKey::from_sec1_bytes(&public_key_sec1).map_err(|e| {
      Error::new(
        Status::InvalidArg,
        format!("Invalid SEC1 public key: {}", e),
      )
    })?;

    let signature = Signature::from_slice(&signature_bytes).map_err(|e| {
      Error::new(Status::InvalidArg, format!("Invalid signature: {}", e))
    })?;

    Ok(verifying_key.verify(&self.data, &signature).is_ok())
  }
}
