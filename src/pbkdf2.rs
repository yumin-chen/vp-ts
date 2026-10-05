use std::num::NonZeroU32;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::pbkdf2;

#[napi(js_name = "pbkdf2Sync")]
pub fn pbkdf2_sync(
  password: Either<Buffer, String>,
  salt: Either<Buffer, String>,
  iterations: u32,
  keylen: u32,
  digest: String,
) -> Result<Buffer> {
  let password_bytes = match password {
    Either::A(b) => b.as_ref().to_vec(),
    Either::B(s) => s.as_bytes().to_vec(),
  };
  let salt_bytes = match salt {
    Either::A(b) => b.as_ref().to_vec(),
    Either::B(s) => s.as_bytes().to_vec(),
  };

  let iter_nonzero = NonZeroU32::new(iterations)
    .ok_or_else(|| Error::from_reason("Iterations must be greater than zero"))?;

  let digest_lower = digest.to_lowercase().replace('-', "");
  let alg = match digest_lower.as_str() {
    "sha1" => pbkdf2::PBKDF2_HMAC_SHA1,
    "sha256" => pbkdf2::PBKDF2_HMAC_SHA256,
    "sha384" => pbkdf2::PBKDF2_HMAC_SHA384,
    "sha512" => pbkdf2::PBKDF2_HMAC_SHA512,
    other => return Err(Error::from_reason(format!("Unsupported digest algorithm: {other}"))),
  };

  let mut out = vec![0u8; keylen as usize];
  pbkdf2::derive(alg, iter_nonzero, &salt_bytes, &password_bytes, &mut out);

  Ok(Buffer::from(out))
}

#[napi(js_name = "pbkdf2")]
pub fn pbkdf2(
  password: Either<Buffer, String>,
  salt: Either<Buffer, String>,
  iterations: u32,
  keylen: u32,
  digest: String,
) -> Result<Buffer> {
  pbkdf2_sync(password, salt, iterations, keylen, digest)
}
