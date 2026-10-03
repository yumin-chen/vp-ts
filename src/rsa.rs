use napi::bindgen_prelude::*;
use napi_derive::napi;
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey};
use rsa::RsaPrivateKey;

#[napi(object)]
pub struct KeyPairResult {
  pub public_key: String,
  pub private_key: String,
}

#[napi]
pub fn generate_key_pair_sync(
  key_type: String,
  modulus_length: Option<u32>,
) -> Result<KeyPairResult> {
  if key_type.to_lowercase() != "rsa" {
    return Err(Error::new(
      Status::InvalidArg,
      format!("Unsupported key type: {}. Currently supported: rsa", key_type),
    ));
  }

  let bits = modulus_length.unwrap_or(2048) as usize;
  let mut rng = rsa::rand_core::OsRng;

  let priv_key = RsaPrivateKey::new(&mut rng, bits)
    .map_err(|e| Error::new(Status::GenericFailure, format!("RSA generation failed: {}", e)))?;

  let pub_key = priv_key.to_public_key();

  let priv_pem = priv_key
    .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
    .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to export private key: {}", e)))?;

  let pub_pem = pub_key
    .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
    .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to export public key: {}", e)))?;

  Ok(KeyPairResult {
    public_key: pub_pem.to_string(),
    private_key: priv_pem.to_string(),
  })
}

pub struct RsaGenTask {
  key_type: String,
  modulus_length: Option<u32>,
}

#[napi]
impl Task for RsaGenTask {
  type Output = KeyPairResult;
  type JsValue = KeyPairResult;

  fn compute(&mut self) -> Result<Self::Output> {
    generate_key_pair_sync(self.key_type.clone(), self.modulus_length)
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi(ts_return_type = "Promise<KeyPairResult>")]
pub fn generate_key_pair(
  key_type: String,
  modulus_length: Option<u32>,
) -> AsyncTask<RsaGenTask> {
  AsyncTask::new(RsaGenTask {
    key_type,
    modulus_length,
  })
}
