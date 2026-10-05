use std::num::NonZeroU32;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::pbkdf2::{
  derive, Algorithm, PBKDF2_HMAC_SHA1, PBKDF2_HMAC_SHA256, PBKDF2_HMAC_SHA384, PBKDF2_HMAC_SHA512,
};
use scrypt::{scrypt as scrypt_kdf, Params};

use crate::hmac::decode_input;

fn get_pbkdf2_algorithm(digest: &str) -> napi::Result<Algorithm> {
  match digest.to_lowercase().replace("-", "").as_str() {
    "sha1" => Ok(PBKDF2_HMAC_SHA1),
    "sha256" => Ok(PBKDF2_HMAC_SHA256),
    "sha384" => Ok(PBKDF2_HMAC_SHA384),
    "sha512" => Ok(PBKDF2_HMAC_SHA512),
    _ => Err(napi::Error::from_reason(format!("Unsupported PBKDF2 digest algorithm: {}", digest))),
  }
}

#[napi(
  ts_args_type = "password: string | Buffer, salt: string | Buffer, iterations: number, keylen: number, digest: string"
)]
pub fn pbkdf2_sync(
  password: Either<String, Buffer>,
  salt: Either<String, Buffer>,
  iterations: u32,
  keylen: u32,
  digest: String,
) -> napi::Result<Buffer> {
  let algo = get_pbkdf2_algorithm(&digest)?;
  let iter = NonZeroU32::new(iterations)
    .ok_or_else(|| napi::Error::from_reason("Iterations must be greater than 0"))?;

  let pass_bytes = decode_input(&password, None);
  let salt_bytes = decode_input(&salt, None);

  let mut out = vec![0u8; keylen as usize];
  derive(algo, iter, &salt_bytes, &pass_bytes, &mut out);

  Ok(Buffer::from(out))
}

#[napi(
  ts_args_type = "password: string | Buffer, salt: string | Buffer, iterations: number, keylen: number, digest: string"
)]
pub fn pbkdf2(
  password: Either<String, Buffer>,
  salt: Either<String, Buffer>,
  iterations: u32,
  keylen: u32,
  digest: String,
) -> napi::Result<Buffer> {
  pbkdf2_sync(password, salt, iterations, keylen, digest)
}

#[napi(
  ts_args_type = "password: string | Buffer, salt: string | Buffer, keylen: number"
)]
pub fn scrypt_sync(
  password: Either<String, Buffer>,
  salt: Either<String, Buffer>,
  keylen: u32,
) -> napi::Result<Buffer> {
  let pass_bytes = decode_input(&password, None);
  let salt_bytes = decode_input(&salt, None);
  let params = Params::new(14, 8, 1, keylen as usize)
    .map_err(|e| napi::Error::from_reason(format!("Invalid scrypt params: {}", e)))?;
  let mut out = vec![0u8; keylen as usize];
  scrypt_kdf(&pass_bytes, &salt_bytes, &params, &mut out)
    .map_err(|e| napi::Error::from_reason(format!("Scrypt derivation failed: {}", e)))?;
  Ok(Buffer::from(out))
}

#[napi(
  ts_args_type = "password: string | Buffer, salt: string | Buffer, keylen: number"
)]
pub fn scrypt(
  password: Either<String, Buffer>,
  salt: Either<String, Buffer>,
  keylen: u32,
) -> napi::Result<Buffer> {
  scrypt_sync(password, salt, keylen)
}

#[napi(js_name = "PBKDF2")]
pub struct PBKDF2 {
  digest: String,
  iterations: u32,
}

#[napi]
impl PBKDF2 {
  #[napi(constructor)]
  pub fn new(digest: String, iterations: u32) -> Self {
    PBKDF2 { digest, iterations }
  }

  #[napi(ts_args_type = "password: string | Buffer, salt: string | Buffer, keylen: number")]
  pub fn derive_sync(
    &self,
    password: Either<String, Buffer>,
    salt: Either<String, Buffer>,
    keylen: u32,
  ) -> napi::Result<Buffer> {
    pbkdf2_sync(password, salt, self.iterations, keylen, self.digest.clone())
  }
}
