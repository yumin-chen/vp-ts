#![deny(clippy::all)]

use crate::key_object::KeyObject;
use ed25519_dalek::{Signature as EdSignature, Verifier, VerifyingKey};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::digest;
use ring::rand::SecureRandom;
use rsa::{
  pkcs8::DecodePublicKey,
  pkcs1v15::{Signature as RsaSignature, VerifyingKey as RsaVerifyingKey},
  signature::Verifier as RsaVerifier,
  RsaPublicKey,
};
use rsa::sha2::Sha256;

#[napi]
pub fn create_public_key(key: Either<String, Uint8Array>) -> Result<KeyObject> {
  let bytes = match key {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.as_ref().to_vec(),
  };
  KeyObject::new("public".to_string(), Uint8Array::from(bytes), Some("rsa".to_string()))
}

#[napi]
pub fn create_secret_key(key: Either<String, Uint8Array>) -> Result<KeyObject> {
  let bytes = match key {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.as_ref().to_vec(),
  };
  KeyObject::new("secret".to_string(), Uint8Array::from(bytes), None)
}

#[napi]
pub fn create_mac(_algorithm: String, key: Either<String, Uint8Array>) -> Result<KeyObject> {
  create_secret_key(key)
}

#[napi]
pub struct Verify {
  pub algorithm: String,
  buffer: Vec<u8>,
}

#[napi]
impl Verify {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> Self {
    Self {
      algorithm,
      buffer: Vec::new(),
    }
  }

  #[napi]
  pub fn update(&mut self, data: Either<String, Uint8Array>) -> Result<()> {
    match data {
      Either::A(s) => self.buffer.extend_from_slice(s.as_bytes()),
      Either::B(b) => self.buffer.extend_from_slice(b.as_ref()),
    }
    Ok(())
  }

  #[napi]
  pub fn verify(
    &mut self,
    key: Either<String, &KeyObject>,
    signature: Either<String, Uint8Array>,
  ) -> Result<bool> {
    let key_bytes = match key {
      Either::A(s) => s.into_bytes(),
      Either::B(k) => k.export().to_vec(),
    };
    let sig_bytes = match signature {
      Either::A(s) => s.into_bytes(),
      Either::B(b) => b.as_ref().to_vec(),
    };

    // Try Ed25519 verification first if 32-byte key and 64-byte signature
    if key_bytes.len() == 32 && sig_bytes.len() == 64 {
      if let Ok(key_array) = key_bytes.as_slice().try_into() {
        if let Ok(verifying_key) = VerifyingKey::from_bytes(key_array) {
          if let Ok(sig_array) = sig_bytes.as_slice().try_into() {
            let ed_sig = EdSignature::from_bytes(sig_array);
            if verifying_key.verify(&self.buffer, &ed_sig).is_ok() {
              return Ok(true);
            }
          }
        }
      }
    }

    if let Ok(pem_str) = String::from_utf8(key_bytes.clone()) {
      if let Ok(pub_key) = RsaPublicKey::from_public_key_pem(&pem_str) {
        let verifying_key = RsaVerifyingKey::<Sha256>::new(pub_key);
        if let Ok(rsa_sig) = RsaSignature::try_from(sig_bytes.as_slice()) {
          Ok(verifying_key.verify(&self.buffer, &rsa_sig).is_ok())
        } else {
          Ok(false)
        }
      } else {
        // Fallback HMAC verification
        let tag_res = ring::hmac::verify(
          &ring::hmac::Key::new(ring::hmac::HMAC_SHA256, &key_bytes),
          &self.buffer,
          &sig_bytes,
        );
        Ok(tag_res.is_ok())
      }
    } else {
      let tag_res = ring::hmac::verify(
        &ring::hmac::Key::new(ring::hmac::HMAC_SHA256, &key_bytes),
        &self.buffer,
        &sig_bytes,
      );
      Ok(tag_res.is_ok())
    }
  }
}

#[napi]
pub fn create_verify(algorithm: String) -> Verify {
  Verify::new(algorithm)
}

#[napi(object)]
pub struct EncapsulateResult {
  pub shared_key: Buffer,
  pub ciphertext: Buffer,
}

#[napi]
pub fn encapsulate(key: Either<String, &KeyObject>) -> Result<EncapsulateResult> {
  let key_bytes = match key {
    Either::A(s) => s.into_bytes(),
    Either::B(k) => k.export().to_vec(),
  };

  let mut ephemeral_nonce = vec![0u8; 16];
  ring::rand::SystemRandom::new()
    .fill(&mut ephemeral_nonce)
    .map_err(|_| Error::new(Status::GenericFailure, "Failed to generate KEM nonce"))?;

  let mut shared_input = key_bytes.clone();
  shared_input.extend_from_slice(&ephemeral_nonce);
  let shared = digest::digest(&digest::SHA256, &shared_input);

  Ok(EncapsulateResult {
    shared_key: Buffer::from(shared.as_ref()),
    ciphertext: Buffer::from(ephemeral_nonce),
  })
}

#[napi]
pub fn decapsulate(
  key: Either<String, &KeyObject>,
  ciphertext: Uint8Array,
) -> Result<Buffer> {
  let key_bytes = match key {
    Either::A(s) => s.into_bytes(),
    Either::B(k) => k.export().to_vec(),
  };

  let mut shared_input = key_bytes;
  shared_input.extend_from_slice(ciphertext.as_ref());
  let shared = digest::digest(&digest::SHA256, &shared_input);

  Ok(Buffer::from(shared.as_ref()))
}
