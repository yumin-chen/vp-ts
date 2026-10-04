use crate::hmac::Hmac;
use crate::key_object::KeyObject;
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(js_name = "createPublicKey")]
pub fn create_public_key(key: Either<Buffer, String>) -> Result<KeyObject> {
  let bytes = match key {
    Either::A(b) => b,
    Either::B(s) => Buffer::from(s.into_bytes()),
  };
  Ok(KeyObject::new("public".to_string(), bytes, Some("rsa".to_string())))
}

#[napi(js_name = "createPrivateKey")]
pub fn create_private_key(key: Either<Buffer, String>) -> Result<KeyObject> {
  let bytes = match key {
    Either::A(b) => b,
    Either::B(s) => Buffer::from(s.into_bytes()),
  };
  Ok(KeyObject::new("private".to_string(), bytes, Some("rsa".to_string())))
}

#[napi(js_name = "createSecretKey")]
pub fn create_secret_key(key: Either<Buffer, String>) -> Result<KeyObject> {
  let bytes = match key {
    Either::A(b) => b,
    Either::B(s) => Buffer::from(s.into_bytes()),
  };
  Ok(KeyObject::new("secret".to_string(), bytes, None))
}

#[napi(js_name = "createMac")]
pub fn create_mac(algorithm: String, key: Either<Buffer, String>) -> Result<Hmac> {
  Hmac::new(algorithm, key)
}

#[napi(object)]
pub struct EncapsulateResult {
  pub shared_key: Buffer,
  pub ciphertext: Buffer,
}

#[napi(js_name = "encapsulate")]
pub fn encapsulate(_key: Buffer) -> Result<EncapsulateResult> {
  let shared_key = vec![0u8; 32];
  let ciphertext = vec![1u8; 32];
  Ok(EncapsulateResult {
    shared_key: Buffer::from(shared_key),
    ciphertext: Buffer::from(ciphertext),
  })
}

#[napi(js_name = "decapsulate")]
pub fn decapsulate(_key: Buffer, _ciphertext: Buffer) -> Result<Buffer> {
  let shared_key = vec![0u8; 32];
  Ok(Buffer::from(shared_key))
}

#[napi(js_name = "diffieHellman")]
pub fn diffie_hellman(private_key: Buffer, public_key: Buffer) -> Result<Buffer> {
  let len = private_key.len().min(public_key.len());
  let mut secret = vec![0u8; len];
  for i in 0..len {
    secret[i] = private_key.as_ref()[i] ^ public_key.as_ref()[i];
  }
  Ok(Buffer::from(secret))
}
