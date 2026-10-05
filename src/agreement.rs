use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub fn diffie_hellman_secret(private_key: Buffer, public_key: Buffer) -> Result<Buffer> {
  if private_key.len() != public_key.len() {
    return Err(Error::from_reason("Key length mismatch"));
  }
  let secret: Vec<u8> = private_key
    .iter()
    .zip(public_key.iter())
    .map(|(a, b)| a ^ b)
    .collect();
  Ok(Buffer::from(secret))
}
