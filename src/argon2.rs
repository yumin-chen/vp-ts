use argon2::{Algorithm, Argon2, Params, Version};
use napi::bindgen_prelude::*;
use napi_derive::napi;

use crate::hmac::decode_input;

#[napi(object)]
pub struct Argon2Parameters {
  pub message: Either<String, Buffer>,
  pub nonce: Either<String, Buffer>,
  pub parallelism: u32,
  pub tag_length: u32,
  pub memory: u32,
  pub passes: u32,
  pub secret: Option<Either<String, Buffer>>,
  pub associated_data: Option<Either<String, Buffer>>,
}

#[napi(ts_args_type = "algorithm: string, parameters: Argon2Parameters")]
pub fn argon2_sync(algorithm: String, parameters: Argon2Parameters) -> napi::Result<Buffer> {
  let pass_bytes = decode_input(&parameters.message, None);
  let salt_bytes = decode_input(&parameters.nonce, None);
  let secret_bytes = parameters.secret.as_ref().map(|s| decode_input(s, None));

  if salt_bytes.len() < 8 {
    return Err(napi::Error::from_reason("Argon2 nonce must be at least 8 bytes"));
  }

  let m_cost = parameters.memory;
  let t_cost = parameters.passes;
  let p_cost = parameters.parallelism;
  let output_len = parameters.tag_length as usize;

  let algo = match algorithm.to_lowercase().as_str() {
    "argon2d" => Algorithm::Argon2d,
    "argon2i" => Algorithm::Argon2i,
    _ => Algorithm::Argon2id,
  };

  let params = Params::new(m_cost, t_cost, p_cost, Some(output_len))
    .map_err(|e| napi::Error::from_reason(format!("Argon2 params error: {}", e)))?;

  let argon2_inst = if let Some(sec) = &secret_bytes {
    Argon2::new_with_secret(sec, algo, Version::V0x13, params)
      .map_err(|e| napi::Error::from_reason(format!("Argon2 secret error: {}", e)))?
  } else {
    Argon2::new(algo, Version::V0x13, params)
  };

  let mut out = vec![0u8; output_len];
  argon2_inst
    .hash_password_into(&pass_bytes, &salt_bytes, &mut out)
    .map_err(|e| napi::Error::from_reason(format!("Argon2 hash error: {}", e)))?;

  Ok(Buffer::from(out))
}

#[napi(ts_args_type = "algorithm: string, parameters: Argon2Parameters")]
pub fn argon2(algorithm: String, parameters: Argon2Parameters) -> napi::Result<Buffer> {
  argon2_sync(algorithm, parameters)
}
