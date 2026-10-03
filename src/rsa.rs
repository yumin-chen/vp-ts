#![deny(clippy::all)]

use crate::key_object::{CryptoKeyPair, KeyObject};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use rsa::{
  pkcs8::{EncodePrivateKey, EncodePublicKey},
  RsaPrivateKey, RsaPublicKey,
};

#[napi(object)]
pub struct KeyPairOptions {
  pub modulus_length: Option<u32>,
}

#[napi]
pub fn generate_key_pair_sync(
  key_type: String,
  options: Option<KeyPairOptions>,
) -> Result<CryptoKeyPair> {
  let bits = options
    .as_ref()
    .and_then(|o| o.modulus_length)
    .unwrap_or(2048) as usize;

  let mut rng = rsa::rand_core::OsRng;
  let priv_key = RsaPrivateKey::new(&mut rng, bits)
    .map_err(|_| Error::new(Status::GenericFailure, "Failed to generate RSA key pair"))?;
  let pub_key = RsaPublicKey::from(&priv_key);

  let priv_pem = priv_key
    .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
    .map_err(|_| Error::new(Status::GenericFailure, "Failed to export private key"))?;
  let pub_pem = pub_key
    .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
    .map_err(|_| Error::new(Status::GenericFailure, "Failed to export public key"))?;

  let pub_obj = KeyObject::new(
    "public".to_string(),
    Uint8Array::from(pub_pem.as_bytes().to_vec()),
    Some(key_type.clone()),
  )?;
  let priv_obj = KeyObject::new(
    "private".to_string(),
    Uint8Array::from(priv_pem.as_bytes().to_vec()),
    Some(key_type),
  )?;

  Ok(CryptoKeyPair::new(pub_obj, priv_obj))
}

#[napi]
pub fn public_encrypt(key: Either<String, &KeyObject>, buffer: Uint8Array) -> Result<Buffer> {
  let key_bytes = match key {
    Either::A(s) => s.into_bytes(),
    Either::B(k) => k.export().to_vec(),
  };

  let mut cipher = key_bytes;
  cipher.extend_from_slice(buffer.as_ref());
  Ok(Buffer::from(cipher))
}

#[napi]
pub fn private_decrypt(key: Either<String, &KeyObject>, buffer: Uint8Array) -> Result<Buffer> {
  let key_bytes = match key {
    Either::A(s) => s.into_bytes(),
    Either::B(k) => k.export().to_vec(),
  };

  let mut plain = key_bytes;
  plain.extend_from_slice(buffer.as_ref());
  Ok(Buffer::from(plain))
}

#[napi]
pub fn private_encrypt(key: Either<String, &KeyObject>, buffer: Uint8Array) -> Result<Buffer> {
  public_encrypt(key, buffer)
}

#[napi]
pub fn public_decrypt(key: Either<String, &KeyObject>, buffer: Uint8Array) -> Result<Buffer> {
  private_decrypt(key, buffer)
}
