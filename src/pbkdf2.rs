use napi::bindgen_prelude::*;
use napi_derive::napi;
use pbkdf2::pbkdf2_hmac;
use sha1::Sha1;
use sha2::{Sha224, Sha256, Sha384, Sha512};

fn run_pbkdf2(
  password: &[u8],
  salt: &[u8],
  iterations: u32,
  keylen: usize,
  digest: &str,
) -> Result<Vec<u8>> {
  let algo_clean = digest.to_lowercase().replace('-', "");
  let mut output = vec![0u8; keylen];

  match algo_clean.as_str() {
    "sha1" => pbkdf2_hmac::<Sha1>(password, salt, iterations, &mut output),
    "sha224" => pbkdf2_hmac::<Sha224>(password, salt, iterations, &mut output),
    "sha256" => pbkdf2_hmac::<Sha256>(password, salt, iterations, &mut output),
    "sha384" => pbkdf2_hmac::<Sha384>(password, salt, iterations, &mut output),
    "sha512" => pbkdf2_hmac::<Sha512>(password, salt, iterations, &mut output),
    _ => return Err(Error::new(Status::InvalidArg, format!("Unsupported PBKDF2 digest: {}", digest))),
  };

  Ok(output)
}

#[napi]
pub fn pbkdf2_sync(
  password: Buffer,
  salt: Buffer,
  iterations: u32,
  keylen: u32,
  digest: String,
) -> Result<Buffer> {
  let res = run_pbkdf2(&password, &salt, iterations, keylen as usize, &digest)?;
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
  let res = run_pbkdf2(&password, &salt, iterations, keylen as usize, &digest)?;
  Ok(Buffer::from(res))
}
