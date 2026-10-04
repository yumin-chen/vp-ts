use napi::bindgen_prelude::*;
use napi_derive::napi;
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

#[napi]
pub fn pbkdf2_sync(
  password: Buffer,
  salt: Buffer,
  iterations: u32,
  keylen: u32,
) -> Buffer {
  let mut buf = vec![0u8; keylen as usize];
  pbkdf2_hmac::<Sha256>(&password, &salt, iterations, &mut buf);
  Buffer::from(buf)
}
