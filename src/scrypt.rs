use napi::bindgen_prelude::*;
use napi_derive::napi;
use scrypt_kdf as scrypt_impl;

#[napi(object)]
#[derive(Default)]
pub struct ScryptOptions {
  pub cost: Option<u32>,
  pub block_size: Option<u32>,
  pub parallelization: Option<u32>,
  pub maxmem: Option<u32>,
}

#[napi(js_name = "scryptSync")]
pub fn scrypt_sync(
  password: Either<Buffer, String>,
  salt: Either<Buffer, String>,
  keylen: u32,
  options: Option<ScryptOptions>,
) -> Result<Buffer> {
  let pass_bytes = match password {
    Either::A(b) => b.as_ref().to_vec(),
    Either::B(s) => s.as_bytes().to_vec(),
  };
  let salt_bytes = match salt {
    Either::A(b) => b.as_ref().to_vec(),
    Either::B(s) => s.as_bytes().to_vec(),
  };

  let opts = options.unwrap_or_default();
  let n = opts.cost.unwrap_or(16384);
  let r = opts.block_size.unwrap_or(8);
  let p = opts.parallelization.unwrap_or(1);

  let log_n = (n as f64).log2() as u8;

  let params = scrypt_impl::Params::new(log_n, r, p, scrypt_impl::Params::RECOMMENDED_LEN)
    .map_err(|e| Error::from_reason(format!("Invalid scrypt params: {e}")))?;

  let mut output = vec![0u8; keylen as usize];
  scrypt_impl::scrypt(&pass_bytes, &salt_bytes, &params, &mut output)
    .map_err(|e| Error::from_reason(format!("scrypt derivation error: {e}")))?;

  Ok(Buffer::from(output))
}

#[napi(js_name = "scrypt")]
pub fn scrypt(
  password: Either<Buffer, String>,
  salt: Either<Buffer, String>,
  keylen: u32,
  options: Option<ScryptOptions>,
) -> Result<Buffer> {
  scrypt_sync(password, salt, keylen, options)
}
