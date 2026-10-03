use aes::Aes128;
use aes_gcm::{
  aead::{Aead, KeyInit, Payload},
  Aes128Gcm, Aes256Gcm, Nonce as GcmNonce,
};
use aes_gcm_siv::{Aes128GcmSiv, Aes256GcmSiv};
use aes_siv::{Aes128SivAead, Aes256SivAead};
use ccm::{
  aead::generic_array::typenum::{U12, U16},
  Ccm,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

type Aes128Ccm = Ccm<Aes128, U16, U12>;

#[napi(object)]
pub struct AeadEncryptResult {
  pub ciphertext: Buffer,
  pub tag: Buffer,
}

#[napi]
pub fn aead_encrypt(
  algorithm: String,
  key: Buffer,
  iv: Buffer,
  plaintext: Buffer,
  aad: Option<Buffer>,
) -> Result<AeadEncryptResult> {
  let alg = algorithm.to_lowercase();
  let aad_bytes = aad.as_ref().map(|b| b.as_ref()).unwrap_or(&[]);
  let payload = Payload {
    msg: plaintext.as_ref(),
    aad: aad_bytes,
  };

  match alg.as_str() {
    "aes-128-gcm" | "aes-256-gcm" => {
      let res = if alg == "aes-128-gcm" {
        let cipher = Aes128Gcm::new_from_slice(key.as_ref())
          .map_err(|e| Error::from_reason(e.to_string()))?;
        let nonce = GcmNonce::from_slice(iv.as_ref());
        cipher.encrypt(nonce, payload)
      } else {
        let cipher = Aes256Gcm::new_from_slice(key.as_ref())
          .map_err(|e| Error::from_reason(e.to_string()))?;
        let nonce = GcmNonce::from_slice(iv.as_ref());
        cipher.encrypt(nonce, payload)
      }
      .map_err(|e| Error::from_reason(e.to_string()))?;

      let split_at = res.len().saturating_sub(16);
      Ok(AeadEncryptResult {
        ciphertext: Buffer::from(res[..split_at].to_vec()),
        tag: Buffer::from(res[split_at..].to_vec()),
      })
    }
    "aes-128-gcm-siv" | "aes-256-gcm-siv" => {
      let res = if alg == "aes-128-gcm-siv" {
        let cipher = Aes128GcmSiv::new_from_slice(key.as_ref())
          .map_err(|e| Error::from_reason(e.to_string()))?;
        let nonce = aes_gcm_siv::Nonce::from_slice(iv.as_ref());
        cipher.encrypt(nonce, payload)
      } else {
        let cipher = Aes256GcmSiv::new_from_slice(key.as_ref())
          .map_err(|e| Error::from_reason(e.to_string()))?;
        let nonce = aes_gcm_siv::Nonce::from_slice(iv.as_ref());
        cipher.encrypt(nonce, payload)
      }
      .map_err(|e| Error::from_reason(e.to_string()))?;

      let split_at = res.len().saturating_sub(16);
      Ok(AeadEncryptResult {
        ciphertext: Buffer::from(res[..split_at].to_vec()),
        tag: Buffer::from(res[split_at..].to_vec()),
      })
    }
    "aes-128-siv" | "aes-256-siv" => {
      let res = if alg == "aes-128-siv" {
        let cipher = Aes128SivAead::new_from_slice(key.as_ref())
          .map_err(|e| Error::from_reason(e.to_string()))?;
        let nonce = aes_siv::Nonce::from_slice(iv.as_ref());
        cipher.encrypt(nonce, payload)
      } else {
        let cipher = Aes256SivAead::new_from_slice(key.as_ref())
          .map_err(|e| Error::from_reason(e.to_string()))?;
        let nonce = aes_siv::Nonce::from_slice(iv.as_ref());
        cipher.encrypt(nonce, payload)
      }
      .map_err(|e| Error::from_reason(e.to_string()))?;

      let split_at = res.len().saturating_sub(16);
      Ok(AeadEncryptResult {
        ciphertext: Buffer::from(res[..split_at].to_vec()),
        tag: Buffer::from(res[split_at..].to_vec()),
      })
    }
    "aes-128-ccm" => {
      let cipher = Aes128Ccm::new_from_slice(key.as_ref())
        .map_err(|e| Error::from_reason(e.to_string()))?;
      let nonce = ccm::Nonce::from_slice(iv.as_ref());
      let res = cipher
        .encrypt(nonce, payload)
        .map_err(|e| Error::from_reason(e.to_string()))?;

      let split_at = res.len().saturating_sub(16);
      Ok(AeadEncryptResult {
        ciphertext: Buffer::from(res[..split_at].to_vec()),
        tag: Buffer::from(res[split_at..].to_vec()),
      })
    }
    _ => Err(Error::from_reason(format!("Unsupported AEAD algorithm: {algorithm}"))),
  }
}

#[napi]
pub fn aead_decrypt(
  algorithm: String,
  key: Buffer,
  iv: Buffer,
  ciphertext: Buffer,
  tag: Buffer,
  aad: Option<Buffer>,
) -> Result<Buffer> {
  let alg = algorithm.to_lowercase();
  let aad_bytes = aad.as_ref().map(|b| b.as_ref()).unwrap_or(&[]);

  let mut combined = ciphertext.as_ref().to_vec();
  combined.extend_from_slice(tag.as_ref());

  let payload = Payload {
    msg: &combined,
    aad: aad_bytes,
  };

  match alg.as_str() {
    "aes-128-gcm" | "aes-256-gcm" => {
      let pt = if alg == "aes-128-gcm" {
        let cipher = Aes128Gcm::new_from_slice(key.as_ref())
          .map_err(|e| Error::from_reason(e.to_string()))?;
        let nonce = GcmNonce::from_slice(iv.as_ref());
        cipher.decrypt(nonce, payload)
      } else {
        let cipher = Aes256Gcm::new_from_slice(key.as_ref())
          .map_err(|e| Error::from_reason(e.to_string()))?;
        let nonce = GcmNonce::from_slice(iv.as_ref());
        cipher.decrypt(nonce, payload)
      }
      .map_err(|e| Error::from_reason(e.to_string()))?;
      Ok(Buffer::from(pt))
    }
    "aes-128-gcm-siv" | "aes-256-gcm-siv" => {
      let pt = if alg == "aes-128-gcm-siv" {
        let cipher = Aes128GcmSiv::new_from_slice(key.as_ref())
          .map_err(|e| Error::from_reason(e.to_string()))?;
        let nonce = aes_gcm_siv::Nonce::from_slice(iv.as_ref());
        cipher.decrypt(nonce, payload)
      } else {
        let cipher = Aes256GcmSiv::new_from_slice(key.as_ref())
          .map_err(|e| Error::from_reason(e.to_string()))?;
        let nonce = aes_gcm_siv::Nonce::from_slice(iv.as_ref());
        cipher.decrypt(nonce, payload)
      }
      .map_err(|e| Error::from_reason(e.to_string()))?;
      Ok(Buffer::from(pt))
    }
    "aes-128-siv" | "aes-256-siv" => {
      let pt = if alg == "aes-128-siv" {
        let cipher = Aes128SivAead::new_from_slice(key.as_ref())
          .map_err(|e| Error::from_reason(e.to_string()))?;
        let nonce = aes_siv::Nonce::from_slice(iv.as_ref());
        cipher.decrypt(nonce, payload)
      } else {
        let cipher = Aes256SivAead::new_from_slice(key.as_ref())
          .map_err(|e| Error::from_reason(e.to_string()))?;
        let nonce = aes_siv::Nonce::from_slice(iv.as_ref());
        cipher.decrypt(nonce, payload)
      }
      .map_err(|e| Error::from_reason(e.to_string()))?;
      Ok(Buffer::from(pt))
    }
    "aes-128-ccm" => {
      let cipher = Aes128Ccm::new_from_slice(key.as_ref())
        .map_err(|e| Error::from_reason(e.to_string()))?;
      let nonce = ccm::Nonce::from_slice(iv.as_ref());
      let pt = cipher
        .decrypt(nonce, payload)
        .map_err(|e| Error::from_reason(e.to_string()))?;
      Ok(Buffer::from(pt))
    }
    _ => Err(Error::from_reason(format!("Unsupported AEAD algorithm: {algorithm}"))),
  }
}
