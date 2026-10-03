#![deny(clippy::all)]

use aes_gcm::{
  aead::{Aead, KeyInit},
  Aes256Gcm, Nonce,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(object)]
pub struct CipherResult {
  pub ciphertext: Buffer,
  pub auth_tag: Buffer,
}

#[napi]
pub fn encrypt_aes_gcm(
  key: Uint8Array,
  iv: Uint8Array,
  plaintext: Uint8Array,
  aad: Option<Uint8Array>,
) -> Result<CipherResult> {
  if key.len() != 32 {
    return Err(Error::new(Status::InvalidArg, "Key length for AES-256-GCM must be 32 bytes"));
  }
  if iv.len() != 12 {
    return Err(Error::new(Status::InvalidArg, "IV length for AES-256-GCM must be 12 bytes"));
  }

  let cipher = Aes256Gcm::new_from_slice(key.as_ref())
    .map_err(|_| Error::new(Status::GenericFailure, "Failed to initialize cipher"))?;
  let nonce = Nonce::from_slice(iv.as_ref());

  let encrypted = if let Some(aad_data) = aad {
    use aes_gcm::aead::Payload;
    cipher
      .encrypt(
        nonce,
        Payload {
          msg: plaintext.as_ref(),
          aad: aad_data.as_ref(),
        },
      )
      .map_err(|_| Error::new(Status::GenericFailure, "Encryption failed"))?
  } else {
    cipher
      .encrypt(nonce, plaintext.as_ref())
      .map_err(|_| Error::new(Status::GenericFailure, "Encryption failed"))?
  };

  // AES-GCM tag is the last 16 bytes
  if encrypted.len() < 16 {
    return Err(Error::new(Status::GenericFailure, "Ciphertext too short"));
  }
  let ct_len = encrypted.len() - 16;
  let ciphertext = encrypted[..ct_len].to_vec();
  let tag = encrypted[ct_len..].to_vec();

  Ok(CipherResult {
    ciphertext: Buffer::from(ciphertext),
    auth_tag: Buffer::from(tag),
  })
}

#[napi]
pub fn decrypt_aes_gcm(
  key: Uint8Array,
  iv: Uint8Array,
  ciphertext: Uint8Array,
  auth_tag: Uint8Array,
  aad: Option<Uint8Array>,
) -> Result<Buffer> {
  if key.len() != 32 {
    return Err(Error::new(Status::InvalidArg, "Key length for AES-256-GCM must be 32 bytes"));
  }
  if iv.len() != 12 {
    return Err(Error::new(Status::InvalidArg, "IV length for AES-256-GCM must be 12 bytes"));
  }

  let cipher = Aes256Gcm::new_from_slice(key.as_ref())
    .map_err(|_| Error::new(Status::GenericFailure, "Failed to initialize cipher"))?;
  let nonce = Nonce::from_slice(iv.as_ref());

  let mut payload_bytes = ciphertext.as_ref().to_vec();
  payload_bytes.extend_from_slice(auth_tag.as_ref());

  let decrypted = if let Some(aad_data) = aad {
    use aes_gcm::aead::Payload;
    cipher
      .decrypt(
        nonce,
        Payload {
          msg: &payload_bytes,
          aad: aad_data.as_ref(),
        },
      )
      .map_err(|_| Error::new(Status::GenericFailure, "Decryption failed"))?
  } else {
    cipher
      .decrypt(nonce, payload_bytes.as_slice())
      .map_err(|_| Error::new(Status::GenericFailure, "Decryption failed"))?
  };

  Ok(Buffer::from(decrypted))
}
