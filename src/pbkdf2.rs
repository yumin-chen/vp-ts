use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::pbkdf2;
use std::num::NonZeroU32;

fn pbkdf2_derive(
  pass_bytes: &[u8],
  salt_bytes: &[u8],
  iterations: u32,
  keylen: u32,
  digest: &str,
) -> Result<Vec<u8>> {
  let algorithm = match digest.to_lowercase().replace('-', "").as_str() {
    "sha1" => pbkdf2::PBKDF2_HMAC_SHA1,
    "sha256" => pbkdf2::PBKDF2_HMAC_SHA256,
    "sha384" => pbkdf2::PBKDF2_HMAC_SHA384,
    "sha512" => pbkdf2::PBKDF2_HMAC_SHA512,
    _ => {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Unsupported digest algorithm: {}", digest),
      ))
    }
  };

  let iter = NonZeroU32::new(iterations).ok_or_else(|| {
    Error::new(
      Status::InvalidArg,
      "Iterations must be greater than zero".to_string(),
    )
  })?;

  let mut out = vec![0u8; keylen as usize];
  pbkdf2::derive(algorithm, iter, salt_bytes, pass_bytes, &mut out);
  Ok(out)
}

fn to_vec(value: Either<String, Uint8Array>) -> Vec<u8> {
  match value {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.as_ref().to_vec(),
  }
}

#[napi]
pub fn pbkdf2_sync(
  #[napi(ts_arg_type = "string | Uint8Array")] password: Either<String, Uint8Array>,
  #[napi(ts_arg_type = "string | Uint8Array")] salt: Either<String, Uint8Array>,
  iterations: u32,
  keylen: u32,
  digest: String,
) -> Result<Buffer> {
  let pass_bytes = to_vec(password);
  let salt_bytes = to_vec(salt);
  let derived = pbkdf2_derive(&pass_bytes, &salt_bytes, iterations, keylen, &digest)?;
  Ok(derived.into())
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
    pbkdf2_derive(
      &self.password,
      &self.salt,
      self.iterations,
      self.keylen,
      &self.digest,
    )
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output.into())
  }
}

#[napi(ts_return_type = "Promise<Buffer>")]
pub fn pbkdf2(
  #[napi(ts_arg_type = "string | Uint8Array")] password: Either<String, Uint8Array>,
  #[napi(ts_arg_type = "string | Uint8Array")] salt: Either<String, Uint8Array>,
  iterations: u32,
  keylen: u32,
  digest: String,
) -> AsyncTask<Pbkdf2Task> {
  AsyncTask::new(Pbkdf2Task {
    password: to_vec(password),
    salt: to_vec(salt),
    iterations,
    keylen,
    digest,
  })
}
