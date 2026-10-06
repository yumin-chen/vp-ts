use napi::bindgen_prelude::*;
use napi_derive::napi;
use x25519_dalek::{StaticSecret, PublicKey as X25519PublicKey};
use ring::rand::{SystemRandom, SecureRandom};
use crate::key_object::KeyObject;

fn random_32_bytes() -> Result<[u8; 32]> {
  let rng = SystemRandom::new();
  let mut bytes = [0u8; 32];
  rng.fill(&mut bytes)
    .map_err(|_| Error::new(Status::GenericFailure, "Random generation failed"))?;
  Ok(bytes)
}

#[napi]
pub fn x25519_generate_keypair() -> Result<Vec<Buffer>> {
  let bytes = random_32_bytes()?;
  let secret = StaticSecret::from(bytes);
  let public = X25519PublicKey::from(&secret);
  Ok(vec![
    Buffer::from(public.as_bytes().to_vec()),
    Buffer::from(secret.to_bytes().to_vec()),
  ])
}

#[napi]
pub fn x25519_diffie_hellman(private_key: Buffer, public_key: Buffer) -> Result<Buffer> {
  let priv_array: [u8; 32] = private_key
    .as_ref()
    .try_into()
    .map_err(|_| Error::new(Status::InvalidArg, "Private key must be 32 bytes"))?;
  let pub_array: [u8; 32] = public_key
    .as_ref()
    .try_into()
    .map_err(|_| Error::new(Status::InvalidArg, "Public key must be 32 bytes"))?;

  let secret = StaticSecret::from(priv_array);
  let public = X25519PublicKey::from(pub_array);
  let shared_secret = secret.diffie_hellman(&public);

  Ok(Buffer::from(shared_secret.as_bytes().to_vec()))
}

#[napi]
pub fn create_public_key(key_pem: String) -> KeyObject {
  KeyObject::new("public".to_string(), Buffer::from(key_pem.into_bytes()), None)
}

#[napi]
pub fn create_private_key(key_pem: String) -> KeyObject {
  KeyObject::new("private".to_string(), Buffer::from(key_pem.into_bytes()), None)
}

#[napi]
pub fn create_secret_key(key_bytes: Buffer) -> KeyObject {
  KeyObject::new("secret".to_string(), key_bytes, None)
}

#[napi(object)]
pub struct EncapsulateResult {
  pub shared_key: Buffer,
  pub ciphertext: Buffer,
}

#[napi]
pub fn encapsulate(public_key: Buffer) -> Result<EncapsulateResult> {
  let bytes = random_32_bytes()?;
  let secret = StaticSecret::from(bytes);
  let pub_array: [u8; 32] = public_key
    .as_ref()
    .try_into()
    .map_err(|_| Error::new(Status::InvalidArg, "Public key must be 32 bytes"))?;
  let remote_public = X25519PublicKey::from(pub_array);
  let shared = secret.diffie_hellman(&remote_public);
  let ephemeral_pub = X25519PublicKey::from(&secret);

  Ok(EncapsulateResult {
    shared_key: Buffer::from(shared.as_bytes().to_vec()),
    ciphertext: Buffer::from(ephemeral_pub.as_bytes().to_vec()),
  })
}

#[napi]
pub fn decapsulate(private_key: Buffer, ciphertext: Buffer) -> Result<Buffer> {
  x25519_diffie_hellman(private_key, ciphertext)
}
