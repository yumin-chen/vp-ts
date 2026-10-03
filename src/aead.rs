use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::aead::{
  BoundKey, UnboundKey, AES_128_GCM, AES_256_GCM, CHACHA20_POLY1305,
};

#[napi]
pub fn aead_encrypt(
  algorithm: String,
  #[napi(ts_arg_type = "Uint8Array")] key: Uint8Array,
  #[napi(ts_arg_type = "Uint8Array")] iv: Uint8Array,
  #[napi(ts_arg_type = "Uint8Array")] plaintext: Uint8Array,
  #[napi(ts_arg_type = "Uint8Array")] aad: Option<Uint8Array>,
) -> Result<Buffer> {
  let algo = match algorithm.to_lowercase().replace('-', "").as_str() {
    "aes128gcm" => &AES_128_GCM,
    "aes256gcm" => &AES_256_GCM,
    "chacha20poly1305" => &CHACHA20_POLY1305,
    _ => {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Unsupported AEAD algorithm: {}", algorithm),
      ))
    }
  };

  let unbound_key = UnboundKey::new(algo, key.as_ref())
    .map_err(|_| Error::new(Status::InvalidArg, "Invalid key length for algorithm"))?;

  let nonce = ring::aead::Nonce::try_assume_unique_for_key(iv.as_ref())
    .map_err(|_| Error::new(Status::InvalidArg, "Invalid IV length for algorithm"))?;

  let aad_bytes = aad.as_ref().map(|a| a.as_ref()).unwrap_or(&[]);
  let aad_struct = ring::aead::Aad::from(aad_bytes);

  let mut in_out = plaintext.as_ref().to_vec();
  let mut key_bound = ring::aead::SealingKey::new(unbound_key, NonceSequence(Some(nonce)));

  key_bound
    .seal_in_place_append_tag(aad_struct, &mut in_out)
    .map_err(|_| Error::new(Status::GenericFailure, "AEAD encryption failed"))?;

  Ok(Buffer::from(in_out))
}

#[napi]
pub fn aead_decrypt(
  algorithm: String,
  #[napi(ts_arg_type = "Uint8Array")] key: Uint8Array,
  #[napi(ts_arg_type = "Uint8Array")] iv: Uint8Array,
  #[napi(ts_arg_type = "Uint8Array")] ciphertext: Uint8Array,
  #[napi(ts_arg_type = "Uint8Array")] aad: Option<Uint8Array>,
) -> Result<Buffer> {
  let algo = match algorithm.to_lowercase().replace('-', "").as_str() {
    "aes128gcm" => &AES_128_GCM,
    "aes256gcm" => &AES_256_GCM,
    "chacha20poly1305" => &CHACHA20_POLY1305,
    _ => {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Unsupported AEAD algorithm: {}", algorithm),
      ))
    }
  };

  let unbound_key = UnboundKey::new(algo, key.as_ref())
    .map_err(|_| Error::new(Status::InvalidArg, "Invalid key length for algorithm"))?;

  let nonce = ring::aead::Nonce::try_assume_unique_for_key(iv.as_ref())
    .map_err(|_| Error::new(Status::InvalidArg, "Invalid IV length for algorithm"))?;

  let aad_bytes = aad.as_ref().map(|a| a.as_ref()).unwrap_or(&[]);
  let aad_struct = ring::aead::Aad::from(aad_bytes);

  let mut in_out = ciphertext.as_ref().to_vec();
  let mut key_bound = ring::aead::OpeningKey::new(unbound_key, NonceSequence(Some(nonce)));

  let decrypted = key_bound
    .open_in_place(aad_struct, &mut in_out)
    .map_err(|_| Error::new(Status::GenericFailure, "AEAD decryption failed"))?;

  Ok(Buffer::from(decrypted.to_vec()))
}

struct NonceSequence(Option<ring::aead::Nonce>);

impl ring::aead::NonceSequence for NonceSequence {
  fn advance(&mut self) -> std::result::Result<ring::aead::Nonce, ring::error::Unspecified> {
    self.0.take().ok_or(ring::error::Unspecified)
  }
}
