#![deny(clippy::all)]

use std::num::NonZeroU32;

use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::pbkdf2;

fn get_pbkdf2_algorithm(digest: &str) -> Result<pbkdf2::Algorithm> {
  let normalized = digest.to_lowercase().replace('-', "");
  match normalized.as_str() {
    "sha1" => Ok(pbkdf2::PBKDF2_HMAC_SHA1),
    "sha256" => Ok(pbkdf2::PBKDF2_HMAC_SHA256),
    "sha384" => Ok(pbkdf2::PBKDF2_HMAC_SHA384),
    "sha512" => Ok(pbkdf2::PBKDF2_HMAC_SHA512),
    _ => Err(Error::new(
      Status::InvalidArg,
      format!("Digest method not supported: {digest}"),
    )),
  }
}

fn input_bytes(input: Either<String, &[u8]>) -> Vec<u8> {
  match input {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.to_vec(),
  }
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
    let algo = get_pbkdf2_algorithm(&self.digest)?;
    let iterations = NonZeroU32::new(self.iterations).ok_or_else(|| {
      Error::new(
        Status::InvalidArg,
        "Iterations must be a non-zero positive integer",
      )
    })?;

    let mut out = vec![0u8; self.keylen as usize];
    pbkdf2::derive(algo, iterations, &self.salt, &self.password, &mut out);
    Ok(out)
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output.into())
  }
}

#[napi(
  ts_args_type = "password: string | Uint8Array, salt: string | Uint8Array, iterations: number, keylen: number, digest: string",
  ts_return_type = "Buffer"
)]
pub fn pbkdf2_sync(
  password: Either<String, &[u8]>,
  salt: Either<String, &[u8]>,
  iterations: u32,
  keylen: u32,
  digest: String,
) -> Result<Buffer> {
  let algo = get_pbkdf2_algorithm(&digest)?;
  let iter_nonzero = NonZeroU32::new(iterations).ok_or_else(|| {
    Error::new(
      Status::InvalidArg,
      "Iterations must be a non-zero positive integer",
    )
  })?;

  let pass_bytes = input_bytes(password);
  let salt_bytes = input_bytes(salt);

  let mut out = vec![0u8; keylen as usize];
  pbkdf2::derive(algo, iter_nonzero, &salt_bytes, &pass_bytes, &mut out);
  Ok(out.into())
}

#[napi(
  ts_args_type = "password: string | Uint8Array, salt: string | Uint8Array, iterations: number, keylen: number, digest: string, abortSignal?: AbortSignal | null",
  ts_return_type = "Promise<Buffer>"
)]
pub fn pbkdf2(
  password: Either<String, &[u8]>,
  salt: Either<String, &[u8]>,
  iterations: u32,
  keylen: u32,
  digest: String,
  abort_signal: Option<AbortSignal>,
) -> AsyncTask<Pbkdf2Task> {
  AsyncTask::with_optional_signal(
    Pbkdf2Task {
      password: input_bytes(password),
      salt: input_bytes(salt),
      iterations,
      keylen,
      digest,
    },
    abort_signal,
  )
}
