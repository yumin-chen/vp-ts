use napi::bindgen_prelude::*;
use napi_derive::napi;
use rsa::{
  pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey},
  Oaep, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
};
use sha1::Sha1;
use sha2::{Sha256, Sha384, Sha512};

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
      format!("Unsupported key pair type: {}", key_type),
    ));
  }

  let bits = modulus_length.unwrap_or(2048) as usize;
  let mut rng = rand::thread_rng();
  let priv_key = RsaPrivateKey::new(&mut rng, bits)
    .map_err(|e| Error::new(Status::GenericFailure, format!("RSA generation failed: {}", e)))?;
  let pub_key = RsaPublicKey::from(&priv_key);

  let priv_pem = priv_key
    .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
    .map_err(|e| Error::new(Status::GenericFailure, format!("PEM export error: {}", e)))?;
  let pub_pem = pub_key
    .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
    .map_err(|e| Error::new(Status::GenericFailure, format!("PEM export error: {}", e)))?;

  Ok(KeyPairResult {
    public_key: pub_pem.to_string(),
    private_key: priv_pem.to_string(),
  })
}

#[napi]
pub async fn generate_key_pair(
  key_type: String,
  modulus_length: Option<u32>,
) -> Result<KeyPairResult> {
  generate_key_pair_sync(key_type, modulus_length)
}

#[napi]
pub fn public_encrypt(key_pem: String, buffer: Buffer, oaep_hash: Option<String>) -> Result<Buffer> {
  let pub_key = RsaPublicKey::from_public_key_pem(&key_pem)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid public key PEM: {}", e)))?;
  let mut rng = rand::thread_rng();

  let encrypted = match oaep_hash.as_deref().map(|s| s.to_lowercase().replace('-', "")).as_deref() {
    Some("sha256") => pub_key.encrypt(&mut rng, Oaep::new::<Sha256>(), &buffer),
    Some("sha384") => pub_key.encrypt(&mut rng, Oaep::new::<Sha384>(), &buffer),
    Some("sha512") => pub_key.encrypt(&mut rng, Oaep::new::<Sha512>(), &buffer),
    Some("sha1") => pub_key.encrypt(&mut rng, Oaep::new::<Sha1>(), &buffer),
    _ => pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &buffer),
  }.map_err(|e| Error::new(Status::GenericFailure, format!("RSA public encrypt failed: {}", e)))?;

  Ok(Buffer::from(encrypted))
}

#[napi]
pub fn private_decrypt(key_pem: String, buffer: Buffer, oaep_hash: Option<String>) -> Result<Buffer> {
  let priv_key = RsaPrivateKey::from_pkcs8_pem(&key_pem)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid private key PEM: {}", e)))?;

  let decrypted = match oaep_hash.as_deref().map(|s| s.to_lowercase().replace('-', "")).as_deref() {
    Some("sha256") => priv_key.decrypt(Oaep::new::<Sha256>(), &buffer),
    Some("sha384") => priv_key.decrypt(Oaep::new::<Sha384>(), &buffer),
    Some("sha512") => priv_key.decrypt(Oaep::new::<Sha512>(), &buffer),
    Some("sha1") => priv_key.decrypt(Oaep::new::<Sha1>(), &buffer),
    _ => priv_key.decrypt(Pkcs1v15Encrypt, &buffer),
  }.map_err(|e| Error::new(Status::GenericFailure, format!("RSA private decrypt failed: {}", e)))?;

  Ok(Buffer::from(decrypted))
}

#[napi]
pub fn private_encrypt(key_pem: String, buffer: Buffer) -> Result<Buffer> {
  let priv_key = RsaPrivateKey::from_pkcs8_pem(&key_pem)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid private key PEM: {}", e)))?;
  let pub_key = RsaPublicKey::from(&priv_key);
  let mut rng = rand::thread_rng();
  let encrypted = pub_key
    .encrypt(&mut rng, Pkcs1v15Encrypt, &buffer)
    .map_err(|e| Error::new(Status::GenericFailure, format!("RSA private encrypt failed: {}", e)))?;
  Ok(Buffer::from(encrypted))
}

#[napi]
pub fn public_decrypt(key_pem: String, buffer: Buffer) -> Result<Buffer> {
  let pub_key = RsaPublicKey::from_public_key_pem(&key_pem)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid public key PEM: {}", e)))?;
  let mut rng = rand::thread_rng();
  let encrypted = pub_key
    .encrypt(&mut rng, Pkcs1v15Encrypt, &buffer)
    .map_err(|e| Error::new(Status::GenericFailure, format!("RSA public decrypt failed: {}", e)))?;
  Ok(Buffer::from(encrypted))
}
