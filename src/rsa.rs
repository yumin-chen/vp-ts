use napi::bindgen_prelude::*;
use napi_derive::napi;
use rsa::pkcs8::EncodePrivateKey;
use rsa::pkcs8::EncodePublicKey;
use rsa::{RsaPrivateKey, RsaPublicKey};

#[napi(object)]
pub struct KeyPairResult {
  pub public_key: String,
  pub private_key: String,
}

#[napi]
pub fn generate_key_pair_sync(bits: u32) -> Result<KeyPairResult> {
  let mut rng = rand::thread_rng();
  let priv_key = RsaPrivateKey::new(&mut rng, bits as usize)
    .map_err(|e| Error::from_reason(e.to_string()))?;
  let pub_key = RsaPublicKey::from(&priv_key);

  let priv_pem = priv_key
    .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
    .map_err(|e| Error::from_reason(e.to_string()))?
    .to_string();

  let pub_pem = pub_key
    .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
    .map_err(|e| Error::from_reason(e.to_string()))?;

  Ok(KeyPairResult {
    public_key: pub_pem,
    private_key: priv_pem,
  })
}
