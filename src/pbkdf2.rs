use std::num::NonZeroU32;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::pbkdf2::{
  derive, PBKDF2_HMAC_SHA1, PBKDF2_HMAC_SHA256, PBKDF2_HMAC_SHA384, PBKDF2_HMAC_SHA512,
  Algorithm,
};

fn get_pbkdf2_alg(digest: &str) -> Result<Algorithm> {
  match digest.to_lowercase().replace("-", "").as_str() {
    "sha1" => Ok(PBKDF2_HMAC_SHA1),
    "sha256" => Ok(PBKDF2_HMAC_SHA256),
    "sha384" => Ok(PBKDF2_HMAC_SHA384),
    "sha512" => Ok(PBKDF2_HMAC_SHA512),
    _ => Err(Error::from_reason(format!("Unsupported PBKDF2 digest: {digest}"))),
  }
}

#[napi]
pub fn pbkdf2_sync(
  password: Either<String, Buffer>,
  salt: Either<String, Buffer>,
  iterations: u32,
  keylen: u32,
  digest: String,
) -> Result<Buffer> {
  let alg = get_pbkdf2_alg(&digest)?;
  let pass_bytes = match password {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.to_vec(),
  };
  let salt_bytes = match salt {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.to_vec(),
  };
  let iter = NonZeroU32::new(iterations)
    .ok_or_else(|| Error::from_reason("Iterations must be greater than 0"))?;

  let mut out = vec![0u8; keylen as usize];
  derive(alg, iter, &salt_bytes, &pass_bytes, &mut out);

  Ok(Buffer::from(out))
}

#[napi]
pub fn pbkdf2(
  password: Either<String, Buffer>,
  salt: Either<String, Buffer>,
  iterations: u32,
  keylen: u32,
  digest: String,
) -> Result<Buffer> {
  pbkdf2_sync(password, salt, iterations, keylen, digest)
}
