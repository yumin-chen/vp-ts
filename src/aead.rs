use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::aead;

#[napi]
pub fn aead_encrypt(
  algorithm: String,
  key: Buffer,
  iv: Buffer,
  plaintext: Buffer,
  aad: Option<Buffer>,
) -> Result<Buffer> {
  let algo = match algorithm.to_lowercase().as_str() {
    "aes-128-gcm" => &aead::AES_128_GCM,
    "aes-256-gcm" => &aead::AES_256_GCM,
    "chacha20-poly1305" => &aead::CHACHA20_POLY1305,
    _ => return Err(Error::new(Status::InvalidArg, format!("Unsupported AEAD algorithm: {}", algorithm))),
  };

  let unbound_key = aead::UnboundKey::new(algo, key.as_ref())
    .map_err(|e| Error::new(Status::InvalidArg, format!("Key error: {:?}", e)))?;
  let less_safe_key = aead::LessSafeKey::new(unbound_key);

  let nonce = aead::Nonce::try_assume_unique_for_key(iv.as_ref())
    .map_err(|e| Error::new(Status::InvalidArg, format!("IV error: {:?}", e)))?;

  let mut in_out = plaintext.as_ref().to_vec();
  let aad_bytes = aad.as_ref().map(|b| b.as_ref()).unwrap_or(&[]);
  let aad_obj = aead::Aad::from(aad_bytes);

  less_safe_key
    .seal_in_place_append_tag(nonce, aad_obj, &mut in_out)
    .map_err(|e| Error::new(Status::GenericFailure, format!("Encryption error: {:?}", e)))?;

  Ok(Buffer::from(in_out))
}
