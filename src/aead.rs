use napi::bindgen_prelude::*;
use napi_derive::napi;
use aes_gcm::{
  aead::{Aead, KeyInit},
  Aes256Gcm, Nonce,
};

#[napi]
pub fn encrypt_gcm(key: Buffer, nonce: Buffer, plaintext: Buffer) -> Result<Buffer> {
  let cipher = Aes256Gcm::new_from_slice(&key)
    .map_err(|e| Error::from_reason(e.to_string()))?;
  let nonce_arr = Nonce::from_slice(&nonce);
  let ciphertext = cipher
    .encrypt(nonce_arr, plaintext.as_ref())
    .map_err(|e| Error::from_reason(e.to_string()))?;
  Ok(Buffer::from(ciphertext))
}

#[napi]
pub fn decrypt_gcm(key: Buffer, nonce: Buffer, ciphertext: Buffer) -> Result<Buffer> {
  let cipher = Aes256Gcm::new_from_slice(&key)
    .map_err(|e| Error::from_reason(e.to_string()))?;
  let nonce_arr = Nonce::from_slice(&nonce);
  let plaintext = cipher
    .decrypt(nonce_arr, ciphertext.as_ref())
    .map_err(|e| Error::from_reason(e.to_string()))?;
  Ok(Buffer::from(plaintext))
}
