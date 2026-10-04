use argon2::{Algorithm, Argon2, Params, Version};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(object)]
pub struct Argon2Options {
  pub memory_cost: Option<u32>,
  pub time_cost: Option<u32>,
  pub parallelism: Option<u32>,
  pub output_length: Option<u32>,
}

#[napi]
pub fn argon2_sync(
  password: Buffer,
  salt: Buffer,
  options: Option<Argon2Options>,
) -> Result<Buffer> {
  let m_cost = options.as_ref().and_then(|o| o.memory_cost).unwrap_or(19456);
  let t_cost = options.as_ref().and_then(|o| o.time_cost).unwrap_or(2);
  let p_cost = options.as_ref().and_then(|o| o.parallelism).unwrap_or(1);
  let out_len = options
    .as_ref()
    .and_then(|o| o.output_length)
    .unwrap_or(32) as usize;

  let params = Params::new(m_cost, t_cost, p_cost, Some(out_len))
    .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid Argon2 params: {}", e)))?;

  let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

  let mut output = vec![0u8; out_len];
  argon2
    .hash_password_into(&password, &salt, &mut output)
    .map_err(|e| Error::new(Status::GenericFailure, format!("Argon2 hash error: {}", e)))?;

  Ok(Buffer::from(output))
}

#[napi]
pub async fn argon2(
  password: Buffer,
  salt: Buffer,
  options: Option<Argon2Options>,
) -> Result<Buffer> {
  argon2_sync(password, salt, options)
}
