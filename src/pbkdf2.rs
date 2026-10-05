use napi::bindgen_prelude::*;
use napi_derive::napi;
use pbkdf2::pbkdf2_hmac;
use sha2::{Sha256, Sha512};

#[napi]
pub fn pbkdf2_sync(
  password: Buffer,
  salt: Buffer,
  iterations: u32,
  keylen: u32,
  digest: String,
) -> Result<Buffer> {
  let mut res = vec![0u8; keylen as usize];

  match digest.to_lowercase().as_str() {
    "sha256" => {
      pbkdf2_hmac::<Sha256>(&password, &salt, iterations, &mut res);
    }
    "sha512" => {
      pbkdf2_hmac::<Sha512>(&password, &salt, iterations, &mut res);
    }
    _ => {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Unsupported PBKDF2 digest algorithm: {}", digest),
      ))
    }  }

  Ok(Buffer::from(res))
}

#[napi]
pub async fn pbkdf2(
  password: Buffer,
  salt: Buffer,
  iterations: u32,
  keylen: u32,
  digest: String,
) -> Result<Buffer> {
  pbkdf2_sync(password, salt, iterations, keylen, digest)
}
