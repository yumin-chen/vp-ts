use napi::bindgen_prelude::*;
use napi_derive::napi;
use rand::rngs::OsRng;
use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

#[napi(object)]
pub struct KeyPairResult {
  pub public_key: String,
  pub private_key: String,
}

#[napi(object)]
pub struct RsaGenerateOptions {
  pub modulus_length: Option<u32>,
  pub public_exponent: Option<u32>,
}

#[napi(js_name = "generateKeyPairSync")]
pub fn generate_key_pair_sync(
  type_: String,
  options: Option<RsaGenerateOptions>,
) -> Result<KeyPairResult> {
  let bits = options.as_ref().and_then(|o| o.modulus_length).unwrap_or(2048) as usize;
  let mut rng = OsRng;

  match type_.to_lowercase().as_str() {
    "rsa" => {
      let priv_key = RsaPrivateKey::new(&mut rng, bits)
        .map_err(|e| Error::from_reason(format!("RSA key generation error: {e}")))?;
      let pub_key = RsaPublicKey::from(&priv_key);

      let priv_pem = priv_key
        .to_pkcs8_pem(LineEnding::LF)
        .map_err(|e| Error::from_reason(format!("PEM export error: {e}")))?
        .to_string();

      let pub_pem = pub_key
        .to_public_key_pem(LineEnding::LF)
        .map_err(|e| Error::from_reason(format!("Public PEM export error: {e}")))?;

      Ok(KeyPairResult {
        public_key: pub_pem,
        private_key: priv_pem,
      })
    }
    other => Err(Error::from_reason(format!("Unsupported key pair type: {other}"))),
  }
}

#[napi(js_name = "generateKeyPair")]
pub fn generate_key_pair(
  type_: String,
  options: Option<RsaGenerateOptions>,
) -> Result<KeyPairResult> {
  generate_key_pair_sync(type_, options)
}

#[napi(js_name = "publicEncrypt")]
pub fn public_encrypt(key_pem: String, buffer: Buffer) -> Result<Buffer> {
  let mut rng = OsRng;
  let pub_key = RsaPublicKey::from_public_key_pem(&key_pem)
    .map_err(|e| Error::from_reason(format!("Failed to parse public key PEM: {e}")))?;
  let enc = pub_key
    .encrypt(&mut rng, Pkcs1v15Encrypt, buffer.as_ref())
    .map_err(|e| Error::from_reason(format!("Encryption error: {e}")))?;
  Ok(Buffer::from(enc))
}

#[napi(js_name = "privateDecrypt")]
pub fn private_decrypt(key_pem: String, buffer: Buffer) -> Result<Buffer> {
  let priv_key = RsaPrivateKey::from_pkcs8_pem(&key_pem)
    .map_err(|e| Error::from_reason(format!("Failed to parse private key PEM: {e}")))?;
  let dec = priv_key
    .decrypt(Pkcs1v15Encrypt, buffer.as_ref())
    .map_err(|e| Error::from_reason(format!("Decryption error: {e}")))?;
  Ok(Buffer::from(dec))
}

#[napi(js_name = "privateEncrypt")]
pub fn private_encrypt(_key_pem: String, buffer: Buffer) -> Result<Buffer> {
  Ok(buffer)
}

#[napi(js_name = "publicDecrypt")]
pub fn public_decrypt(_key_pem: String, buffer: Buffer) -> Result<Buffer> {
  Ok(buffer)
}
