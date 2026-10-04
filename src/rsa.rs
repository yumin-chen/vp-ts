use napi::bindgen_prelude::*;
use napi_derive::napi;
use rand::rngs::OsRng;
use rsa::{
  pkcs8::{EncodePrivateKey, EncodePublicKey},
  RsaPrivateKey,
};

#[napi(object)]
pub struct KeyPairResult {
  pub public_key: String,
  pub private_key: String,
}

#[napi]
pub fn generate_key_pair_sync(bits: Option<u32>) -> Result<KeyPairResult> {
  let modulus_bits = bits.unwrap_or(2048) as usize;
  let mut rng = OsRng;

  let private_key = RsaPrivateKey::new(&mut rng, modulus_bits)
    .map_err(|e| Error::new(Status::GenericFailure, format!("RSA gen error: {}", e)))?;
  let public_key = private_key.to_public_key();

  let priv_pem = private_key
    .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
    .map_err(|e| Error::new(Status::GenericFailure, format!("PEM export error: {}", e)))?;

  let pub_pem = public_key
    .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
    .map_err(|e| Error::new(Status::GenericFailure, format!("PEM export error: {}", e)))?;

  Ok(KeyPairResult {
    public_key: pub_pem.to_string(),
    private_key: priv_pem.to_string(),
  })
}

#[napi]
pub async fn generate_key_pair(bits: Option<u32>) -> Result<KeyPairResult> {
  generate_key_pair_sync(bits)
}
