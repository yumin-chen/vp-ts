#![deny(clippy::all)]

use crate::key_object::{CryptoKeyPair, KeyObject};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use rsa::{
  pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey},
  Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
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

  let pem_str = String::from_utf8(key_bytes)
    .map_err(|_| Error::new(Status::InvalidArg, "Key must be UTF-8 PEM string"))?;
  let pub_key = RsaPublicKey::from_public_key_pem(&pem_str)
    .map_err(|_| Error::new(Status::InvalidArg, "Invalid RSA public key PEM"))?;

  let mut rng = rsa::rand_core::OsRng;
  let encrypted = pub_key
    .encrypt(&mut rng, Pkcs1v15Encrypt, buffer.as_ref())
    .map_err(|_| Error::new(Status::GenericFailure, "RSA public encryption failed"))?;

  Ok(Buffer::from(encrypted))
}

#[napi]
pub fn private_decrypt(key: Either<String, &KeyObject>, buffer: Uint8Array) -> Result<Buffer> {
  let key_bytes = match key {
    Either::A(s) => s.into_bytes(),
    Either::B(k) => k.export().to_vec(),
  };

  let pem_str = String::from_utf8(key_bytes)
    .map_err(|_| Error::new(Status::InvalidArg, "Key must be UTF-8 PEM string"))?;
  let priv_key = RsaPrivateKey::from_pkcs8_pem(&pem_str)
    .map_err(|_| Error::new(Status::InvalidArg, "Invalid RSA private key PEM"))?;

  let decrypted = priv_key
    .decrypt(Pkcs1v15Encrypt, buffer.as_ref())
    .map_err(|_| Error::new(Status::GenericFailure, "RSA private decryption failed"))?;

  Ok(Buffer::from(decrypted))
}

#[napi]
pub fn private_encrypt(key: Either<String, &KeyObject>, buffer: Uint8Array) -> Result<Buffer> {
  let key_bytes = match key {
    Either::A(s) => s.into_bytes(),
    Either::B(k) => k.export().to_vec(),
  };

  let pem_str = String::from_utf8(key_bytes)
    .map_err(|_| Error::new(Status::InvalidArg, "Key must be UTF-8 PEM string"))?;
  let priv_key = RsaPrivateKey::from_pkcs8_pem(&pem_str)
    .map_err(|_| Error::new(Status::InvalidArg, "Invalid RSA private key PEM"))?;

  let pub_key = RsaPublicKey::from(&priv_key);
  let mut rng = rsa::rand_core::OsRng;
  let encrypted = pub_key
    .encrypt(&mut rng, Pkcs1v15Encrypt, buffer.as_ref())
    .map_err(|_| Error::new(Status::GenericFailure, "RSA private encryption failed"))?;

  Ok(Buffer::from(encrypted))
}

#[napi]
pub fn public_decrypt(key: Either<String, &KeyObject>, buffer: Uint8Array) -> Result<Buffer> {
  public_encrypt(key, buffer)
}
