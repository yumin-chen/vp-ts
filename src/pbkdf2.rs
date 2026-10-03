use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::pbkdf2;
use std::num::NonZeroU32;

#[napi]
pub fn pbkdf2_sync(
  password: Buffer,
  salt: Buffer,
  iterations: u32,
  keylen: u32,
  digest: String,
) -> Result<Buffer> {
  let lower = digest.to_lowercase();
  let algo = match lower.as_str() {
    "sha256" => pbkdf2::PBKDF2_HMAC_SHA256,
    "sha384" => pbkdf2::PBKDF2_HMAC_SHA384,
    "sha512" => pbkdf2::PBKDF2_HMAC_SHA512,
    "sha1" => pbkdf2::PBKDF2_HMAC_SHA1,
    _ => {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Unsupported digest algorithm for PBKDF2: {}", digest),
      ))
    }
  };

  let iter = NonZeroU32::new(iterations).ok_or_else(|| {
    Error::new(
      Status::InvalidArg,
      "Iterations must be greater than zero",
    )
  })?;

  let mut out = vec![0u8; keylen as usize];
  pbkdf2::derive(
    algo,
    iter,
    salt.as_ref(),
    password.as_ref(),
    &mut out,
  );

  Ok(Buffer::from(out))
}

pub struct Pbkdf2Task {
  password: Vec<u8>,
  salt: Vec<u8>,
  iterations: u32,
  keylen: u32,
  digest: String,
}

impl Task for Pbkdf2Task {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let lower = self.digest.to_lowercase();
    let algo = match lower.as_str() {
      "sha256" => pbkdf2::PBKDF2_HMAC_SHA256,
      "sha384" => pbkdf2::PBKDF2_HMAC_SHA384,
      "sha512" => pbkdf2::PBKDF2_HMAC_SHA512,
      "sha1" => pbkdf2::PBKDF2_HMAC_SHA1,
      _ => {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Unsupported digest algorithm for PBKDF2: {}", self.digest),
        ))
      }
    };

    let iter = NonZeroU32::new(self.iterations).ok_or_else(|| {
      Error::new(
        Status::InvalidArg,
        "Iterations must be greater than zero",
      )
    })?;

    let mut out = vec![0u8; self.keylen as usize];
    pbkdf2::derive(
      algo,
      iter,
      &self.salt,
      &self.password,
      &mut out,
    );

    Ok(out)
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(Buffer::from(output))
  }
}

#[napi]
pub fn pbkdf2(
  password: Buffer,
  salt: Buffer,
  iterations: u32,
  keylen: u32,
  digest: String,
) -> AsyncTask<Pbkdf2Task> {
  AsyncTask::new(Pbkdf2Task {
    password: password.as_ref().to_vec(),
    salt: salt.as_ref().to_vec(),
    iterations,
    keylen,
    digest,
  })
}

#[napi]
pub struct Pbkdf2 {
  pub iterations: u32,
  pub keylen: u32,
  pub digest: String,
}

#[napi]
impl Pbkdf2 {
  #[napi(constructor)]
  pub fn new(iterations: u32, keylen: u32, digest: String) -> Self {
    Self {
      iterations,
      keylen,
      digest,
    }
  }

  #[napi]
  pub fn derive_sync(&self, password: Buffer, salt: Buffer) -> Result<Buffer> {
    pbkdf2_sync(password, salt, self.iterations, self.keylen, self.digest.clone())
  }

  #[napi]
  pub fn derive(&self, password: Buffer, salt: Buffer) -> AsyncTask<Pbkdf2Task> {
    pbkdf2(password, salt, self.iterations, self.keylen, self.digest.clone())
  }
}
