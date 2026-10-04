#![deny(clippy::all)]

use std::num::NonZeroU32;

use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::pbkdf2;

fn get_pbkdf2_algorithm(name: &str) -> Result<pbkdf2::Algorithm> {
  let normalized = name.to_lowercase().replace(['-', '_'], "");
  match normalized.as_str() {
    "sha1" => Ok(pbkdf2::PBKDF2_HMAC_SHA1),
    "sha256" => Ok(pbkdf2::PBKDF2_HMAC_SHA256),
    "sha384" => Ok(pbkdf2::PBKDF2_HMAC_SHA384),
    "sha512" => Ok(pbkdf2::PBKDF2_HMAC_SHA512),
    _ => Err(Error::new(
      Status::InvalidArg,
      format!("Unsupported PBKDF2 digest algorithm: {name}"),
    )),
  }
}

pub fn derive_pbkdf2(
  password: &[u8],
  salt: &[u8],
  iterations: u32,
  keylen: usize,
  digest: &str,
) -> Result<Vec<u8>> {
  let alg = get_pbkdf2_algorithm(digest)?;
  let iter_nonzero = NonZeroU32::new(iterations).ok_or_else(|| {
    Error::new(
      Status::InvalidArg,
      "Iterations must be greater than 0".to_string(),
    )
  })?;

  let mut out = vec![0u8; keylen];
  pbkdf2::derive(alg, iter_nonzero, salt, password, &mut out);
  Ok(out)
}

#[napi]
pub fn pbkdf2_sync(
  password: Either<String, Uint8Array>,
  salt: Either<String, Uint8Array>,
  iterations: u32,
  keylen: u32,
  digest: String,
) -> Result<Buffer> {
  let pass_bytes = match password {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.as_ref().to_vec(),
  };
  let salt_bytes = match salt {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.as_ref().to_vec(),
  };

  let derived = derive_pbkdf2(
    &pass_bytes,
    &salt_bytes,
    iterations,
    keylen as usize,
    &digest,
  )?;
  Ok(Buffer::from(derived))
}

pub struct Pbkdf2Task {
  password: Vec<u8>,
  salt: Vec<u8>,
  iterations: u32,
  keylen: u32,
  digest: String,
}

#[napi]
impl Task for Pbkdf2Task {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    derive_pbkdf2(
      &self.password,
      &self.salt,
      self.iterations,
      self.keylen as usize,
      &self.digest,
    )
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(Buffer::from(output))
  }
}

#[napi]
pub fn pbkdf2(
  password: Either<String, Uint8Array>,
  salt: Either<String, Uint8Array>,
  iterations: u32,
  keylen: u32,
  digest: String,
  abort_signal: Option<AbortSignal>,
) -> AsyncTask<Pbkdf2Task> {
  let pass_bytes = match password {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.as_ref().to_vec(),
  };
  let salt_bytes = match salt {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.as_ref().to_vec(),
  };

  AsyncTask::with_optional_signal(
    Pbkdf2Task {
      password: pass_bytes,
      salt: salt_bytes,
      iterations,
      keylen,
      digest,
    },
    abort_signal,
  )
}
