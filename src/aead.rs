use napi::bindgen_prelude::*;
use napi_derive::napi;
use aes_gcm::{
  aead::{Aead, KeyInit, Payload},
  Aes128Gcm, Aes256Gcm,
};

#[napi(object)]
pub struct CipherInfo {
  pub name: String,
  pub key_length: u32,
  pub iv_length: Option<u32>,
  pub block_size: Option<u32>,
  pub mode: String,
}

#[napi]
pub fn get_ciphers() -> Vec<String> {
  vec![
    "aes-128-gcm".to_string(),
    "aes-256-gcm".to_string(),
    "chacha20-poly1305".to_string(),
    "aes-128-ccm".to_string(),
    "aes-256-ccm".to_string(),
  ]
}

#[napi]
pub fn get_cipher_info(name: String) -> Option<CipherInfo> {
  let clean = name.to_lowercase().replace('_', "-");
  match clean.as_str() {
    "aes-128-gcm" => Some(CipherInfo {
      name: "aes-128-gcm".to_string(),
      key_length: 16,
      iv_length: Some(12),
      block_size: Some(16),
      mode: "gcm".to_string(),
    }),
    "aes-256-gcm" => Some(CipherInfo {
      name: "aes-256-gcm".to_string(),
      key_length: 32,
      iv_length: Some(12),
      block_size: Some(16),
      mode: "gcm".to_string(),
    }),
    "chacha20-poly1305" => Some(CipherInfo {
      name: "chacha20-poly1305".to_string(),
      key_length: 32,
      iv_length: Some(12),
      block_size: Some(1),
      mode: "stream".to_string(),
    }),
    _ => None,
  }
}

#[napi(object)]
pub struct AeadResult {
  pub ciphertext: Buffer,
  pub tag: Buffer,
}

#[napi]
pub fn encrypt_aead(
  algorithm: String,
  key: Buffer,
  iv: Buffer,
  plaintext: Buffer,
  aad: Option<Buffer>,
) -> Result<AeadResult> {
  let clean = algorithm.to_lowercase().replace('_', "-");
  match clean.as_str() {
    "aes-128-gcm" => {
      let cipher = Aes128Gcm::new_from_slice(&key)
        .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid key length: {}", e)))?;
      let nonce = aes_gcm::Nonce::from_slice(&iv);
      let payload = Payload {
        msg: &plaintext,
        aad: aad.as_deref().unwrap_or(&[]),
      };
      let mut ct = cipher.encrypt(nonce, payload)
        .map_err(|e| Error::new(Status::GenericFailure, format!("Encryption failed: {}", e)))?;
      let tag = ct.split_off(ct.len().saturating_sub(16));
      Ok(AeadResult {
        ciphertext: Buffer::from(ct),
        tag: Buffer::from(tag),
      })
    }
    "aes-256-gcm" => {
      let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid key length: {}", e)))?;
      let nonce = aes_gcm::Nonce::from_slice(&iv);
      let payload = Payload {
        msg: &plaintext,
        aad: aad.as_deref().unwrap_or(&[]),
      };
      let mut ct = cipher.encrypt(nonce, payload)
        .map_err(|e| Error::new(Status::GenericFailure, format!("Encryption failed: {}", e)))?;
      let tag = ct.split_off(ct.len().saturating_sub(16));
      Ok(AeadResult {
        ciphertext: Buffer::from(ct),
        tag: Buffer::from(tag),
      })
    }
    _ => Err(Error::new(Status::InvalidArg, format!("Unsupported AEAD algorithm: {}", algorithm))),
  }
}

#[napi]
pub fn decrypt_aead(
  algorithm: String,
  key: Buffer,
  iv: Buffer,
  ciphertext: Buffer,
  tag: Buffer,
  aad: Option<Buffer>,
) -> Result<Buffer> {
  let clean = algorithm.to_lowercase().replace('_', "-");
  let mut combined = ciphertext.to_vec();
  combined.extend_from_slice(&tag);

  match clean.as_str() {
    "aes-128-gcm" => {
      let cipher = Aes128Gcm::new_from_slice(&key)
        .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid key length: {}", e)))?;
      let nonce = aes_gcm::Nonce::from_slice(&iv);
      let payload = Payload {
        msg: &combined,
        aad: aad.as_deref().unwrap_or(&[]),
      };
      let pt = cipher.decrypt(nonce, payload)
        .map_err(|e| Error::new(Status::GenericFailure, format!("Decryption failed: {}", e)))?;
      Ok(Buffer::from(pt))
    }
    "aes-256-gcm" => {
      let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid key length: {}", e)))?;
      let nonce = aes_gcm::Nonce::from_slice(&iv);
      let payload = Payload {
        msg: &combined,
        aad: aad.as_deref().unwrap_or(&[]),
      };
      let pt = cipher.decrypt(nonce, payload)
        .map_err(|e| Error::new(Status::GenericFailure, format!("Decryption failed: {}", e)))?;
      Ok(Buffer::from(pt))
    }
    _ => Err(Error::new(Status::InvalidArg, format!("Unsupported AEAD algorithm: {}", algorithm))),
  }
}
