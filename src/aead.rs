use aes_gcm::{
  aead::{Aead, KeyInit},
  Aes128Gcm, Aes256Gcm, Nonce,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub fn encrypt_aead(
  algorithm: String,
  key: Buffer,
  iv: Buffer,
  plaintext: Buffer,
  _aad: Option<Buffer>,
) -> Result<Buffer> {
  match algorithm.to_lowercase().as_str() {
    "aes-128-gcm" => {
      let cipher = Aes128Gcm::new_from_slice(&key)
        .map_err(|e| Error::new(Status::InvalidArg, format!("Key error: {}", e)))?;
      let nonce = Nonce::from_slice(&iv);
      let ciphertext = cipher
        .encrypt(nonce, plaintext.as_ref())
        .map_err(|e| Error::new(Status::GenericFailure, format!("Encryption error: {}", e)))?;
      Ok(Buffer::from(ciphertext))
    }
    "aes-256-gcm" => {
      let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| Error::new(Status::InvalidArg, format!("Key error: {}", e)))?;
      let nonce = Nonce::from_slice(&iv);
      let ciphertext = cipher
        .encrypt(nonce, plaintext.as_ref())
        .map_err(|e| Error::new(Status::GenericFailure, format!("Encryption error: {}", e)))?;
      Ok(Buffer::from(ciphertext))
    }
    _ => Err(Error::new(
      Status::InvalidArg,
      format!("Unsupported AEAD algorithm: {}", algorithm),
    )),
  }
}

#[napi]
pub fn decrypt_aead(
  algorithm: String,
  key: Buffer,
  iv: Buffer,
  ciphertext: Buffer,
  _aad: Option<Buffer>,
) -> Result<Buffer> {
  match algorithm.to_lowercase().as_str() {
    "aes-128-gcm" => {
      let cipher = Aes128Gcm::new_from_slice(&key)
        .map_err(|e| Error::new(Status::InvalidArg, format!("Key error: {}", e)))?;
      let nonce = Nonce::from_slice(&iv);
      let plaintext = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .map_err(|e| Error::new(Status::GenericFailure, format!("Decryption error: {}", e)))?;
      Ok(Buffer::from(plaintext))
    }
    "aes-256-gcm" => {
      let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| Error::new(Status::InvalidArg, format!("Key error: {}", e)))?;
      let nonce = Nonce::from_slice(&iv);
      let plaintext = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .map_err(|e| Error::new(Status::GenericFailure, format!("Decryption error: {}", e)))?;
      Ok(Buffer::from(plaintext))
    }
    _ => Err(Error::new(
      Status::InvalidArg,
      format!("Unsupported AEAD algorithm: {}", algorithm),
    )),
  }
}
