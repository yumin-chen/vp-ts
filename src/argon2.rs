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
  let tag_len = params.tag_length.unwrap_or(64) as usize;
  let m_cost = params.memory.unwrap_or(4096); // KiB
  let t_cost = params.passes.unwrap_or(3);

  let nonce_bytes = params.nonce.as_deref().unwrap_or(&[]);
  let message_bytes = params.message.as_deref().unwrap_or(&[]);
  let secret_bytes = params.secret.as_deref().unwrap_or(&[]);

  let argon2_params = Params::new(m_cost, t_cost, p_cost, Some(tag_len))
    .map_err(|e| Error::new(Status::GenericFailure, format!("Argon2 derivation failed: {}", e)))?;

  let argon2_ctx = if !secret_bytes.is_empty() {
    Argon2::new_with_secret(secret_bytes, algo, Version::V0x13, argon2_params)
      .map_err(|e| Error::new(Status::GenericFailure, format!("Argon2 derivation failed: {}", e)))?
  } else {
    Argon2::new(algo, Version::V0x13, argon2_params)
  };

  let mut output = vec![0u8; tag_len];
  argon2_ctx
    .hash_password_into(message_bytes, nonce_bytes, &mut output)
    .map_err(|e| Error::new(Status::GenericFailure, format!("Argon2 derivation failed: {}", e)))?;

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
