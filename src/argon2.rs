use argon2::{Algorithm, Argon2, Params, Version};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(object)]
pub struct Argon2Parameters {
  pub message: Option<Buffer>,
  pub nonce: Option<Buffer>,
  pub parallelism: Option<u32>,
  pub tag_length: Option<u32>,
  pub memory: Option<u32>,
  pub passes: Option<u32>,
  pub secret: Option<Buffer>,
  pub associated_data: Option<Buffer>,
}

fn parse_algorithm(algorithm: &str) -> Result<Algorithm> {
  match algorithm.to_lowercase().as_str() {
    "argon2d" => Ok(Algorithm::Argon2d),
    "argon2i" => Ok(Algorithm::Argon2i),
    "argon2id" => Ok(Algorithm::Argon2id),
    _ => Err(Error::new(
      Status::InvalidArg,
      format!("The argument 'algorithm' must be one of: 'argon2d', 'argon2i', 'argon2id'. Received '{}'", algorithm),
    )),
  }
}

fn run_argon2_impl(algorithm: &str, params: Argon2Parameters) -> Result<Vec<u8>> {
  let algo = parse_algorithm(algorithm)?;

  let p_cost = params.parallelism.unwrap_or(1);
  if p_cost < 1 || p_cost > 16777215 {
    return Err(Error::new(
      Status::InvalidArg,
      format!("The value of \"parameters.parallelism\" is out of range. It must be >= 1 && <= 16777215. Received {}", p_cost),
    ));
  }

  let tag_len = params.tag_length.unwrap_or(64) as usize;
  if tag_len < 4 {
    return Err(Error::new(
      Status::InvalidArg,
      format!("The value of \"parameters.tagLength\" is out of range. It must be >= 4 && <= 4294967295. Received {}", tag_len),
    ));
  }

  let m_cost = params.memory.unwrap_or(4096); // KiB
  if m_cost < 8 {
    return Err(Error::new(
      Status::InvalidArg,
      format!("The value of \"parameters.memory\" is out of range. It must be >= 8 && <= 4294967295. Received {}", m_cost),
    ));
  }

  let t_cost = params.passes.unwrap_or(3);
  if t_cost < 1 {
    return Err(Error::new(
      Status::InvalidArg,
      format!("The value of \"parameters.passes\" is out of range. It must be >= 1 && <= 4294967295. Received {}", t_cost),
    ));
  }

  let nonce_bytes = params.nonce.as_deref().unwrap_or(&[]);
  if nonce_bytes.len() < 8 {
    return Err(Error::new(
      Status::InvalidArg,
      format!("The value of \"parameters.nonce.byteLength\" is out of range. It must be >= 8 && <= 4294967295. Received {}", nonce_bytes.len()),
    ));
  }

  let message_bytes = params.message.as_deref().unwrap_or(&[]);
  let secret_bytes = params.secret.as_deref().unwrap_or(&[]);
  let ad_bytes = params.associated_data.as_deref().unwrap_or(&[]);

  let argon2_params = Params::new(m_cost, t_cost, p_cost, Some(tag_len))
    .map_err(|e| Error::new(Status::GenericFailure, format!("Argon2 params error: {}", e)))?;

  let argon2_ctx = if !secret_bytes.is_empty() || !ad_bytes.is_empty() {
    Argon2::new_with_secret(secret_bytes, algo, Version::V0x13, argon2_params)
      .map_err(|e| Error::new(Status::GenericFailure, format!("Argon2 secret error: {}", e)))?
  } else {
    Argon2::new(algo, Version::V0x13, argon2_params)
  };

  let mut output = vec![0u8; tag_len];
  argon2_ctx
    .hash_password_into(message_bytes, nonce_bytes, &mut output)
    .map_err(|e| Error::new(Status::GenericFailure, format!("Argon2 hash error: {}", e)))?;

  Ok(output)
}

#[napi]
pub fn argon2_sync(algorithm: String, parameters: Argon2Parameters) -> Result<Buffer> {
  let res = run_argon2_impl(&algorithm, parameters)?;
  Ok(Buffer::from(res))
}

#[napi]
pub async fn argon2(algorithm: String, parameters: Argon2Parameters) -> Result<Buffer> {
  let res = run_argon2_impl(&algorithm, parameters)?;
  Ok(Buffer::from(res))
}
