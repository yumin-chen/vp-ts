use napi::bindgen_prelude::*;
use napi_derive::napi;
use argon2::{Argon2, PasswordHasher, PasswordVerifier, PasswordHash};
use argon2::password_hash::SaltString;
use rand::rngs::OsRng;

#[napi]
pub fn argon2_sync(password: Buffer, salt: Buffer) -> String {
  let salt_str = SaltString::encode_b64(&salt).unwrap_or_else(|_| SaltString::generate(&mut OsRng));
  let argon2 = Argon2::default();
  argon2.hash_password(&password, &salt_str)
    .map(|h| h.to_string())
    .unwrap_or_default()
}

#[napi]
pub fn argon2_verify_sync(hash: String, password: Buffer) -> bool {
  if let Ok(parsed_hash) = PasswordHash::new(&hash) {
    Argon2::default().verify_password(&password, &parsed_hash).is_ok()
  } else {
    false
  }
}
