use napi::bindgen_prelude::*;
use napi_derive::napi;
use rsa::pkcs8::DecodePublicKey;
use rsa::signature::Verifier;
use rsa::RsaPublicKey;
use sha2::Sha256;

use crate::hmac::decode_input;

#[napi(object)]
pub struct EncapsulateResult {
  pub shared_key: Buffer,
  pub ciphertext: Buffer,
}

#[napi(ts_args_type = "key: string | Buffer")]
pub fn encapsulate(key: Either<String, Buffer>) -> EncapsulateResult {
  let key_bytes = decode_input(&key, None);
  let rng = ring::rand::SystemRandom::new();
  let mut secret = vec![0u8; 32];
  let _ = ring::rand::SecureRandom::fill(&rng, &mut secret);

  let shared_hash = ring::digest::digest(&ring::digest::SHA256, &secret);

  let mut ciphertext = vec![0u8; secret.len()];
  for (i, (&s, &k)) in secret.iter().zip(key_bytes.iter().cycle()).enumerate() {
    ciphertext[i] = s ^ k;
  }

  EncapsulateResult {
    shared_key: Buffer::from(shared_hash.as_ref().to_vec()),
    ciphertext: Buffer::from(ciphertext),
  }
}

#[napi(ts_args_type = "key: string | Buffer, ciphertext: string | Buffer")]
pub fn decapsulate(key: Either<String, Buffer>, ciphertext: Either<String, Buffer>) -> Buffer {
  let key_bytes = decode_input(&key, None);
  let ct_bytes = decode_input(&ciphertext, None);

  let mut secret = vec![0u8; ct_bytes.len()];
  for (i, (&c, &k)) in ct_bytes.iter().zip(key_bytes.iter().cycle()).enumerate() {
    secret[i] = c ^ k;
  }

  let shared_hash = ring::digest::digest(&ring::digest::SHA256, &secret);
  Buffer::from(shared_hash.as_ref().to_vec())
}

#[napi(js_name = "Verify")]
pub struct Verify {
  pub algorithm: String,
  pub data: Vec<u8>,
}

#[napi]
impl Verify {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> Self {
    Verify {
      algorithm,
      data: Vec::new(),
    }
  }

  #[napi(ts_args_type = "data: string | Buffer, inputEncoding?: string")]
  pub fn update(&mut self, data: Either<String, Buffer>, encoding: Option<String>) {
    let bytes = decode_input(&data, encoding.as_deref());
    self.data.extend_from_slice(&bytes);
  }

  #[napi(ts_args_type = "key: string | Buffer, signature: string | Buffer")]
  pub fn verify(&self, key: Either<String, Buffer>, signature: Either<String, Buffer>) -> bool {
    let key_bytes = decode_input(&key, None);
    let sig_bytes = decode_input(&signature, None);
    let key_str = String::from_utf8_lossy(&key_bytes);

    if let Ok(pub_key) = RsaPublicKey::from_public_key_pem(&key_str) {
      let verifying_key = rsa::pkcs1v15::VerifyingKey::<Sha256>::new_unprefixed(pub_key);
      if let Ok(sig) = rsa::pkcs1v15::Signature::try_from(sig_bytes.as_slice()) {
        verifying_key.verify(&self.data, &sig).is_ok()
      } else {
        false
      }
    } else {
      false
    }
  }
}

#[napi]
pub fn create_verify(algorithm: String) -> Verify {
  Verify::new(algorithm)
}

#[napi(ts_args_type = "algorithm: string, data: string | Buffer, key: string | Buffer, signature: string | Buffer")]
pub fn verify(
  algorithm: String,
  data: Either<String, Buffer>,
  key: Either<String, Buffer>,
  signature: Either<String, Buffer>,
) -> bool {
  let mut v = Verify::new(algorithm);
  v.update(data, None);
  v.verify(key, signature)
}
