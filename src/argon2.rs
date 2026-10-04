use argon2_rust::{Algorithm, Argon2, Params, Version};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(object)]
pub struct Argon2Params {
  pub message: Option<Buffer>,
  pub nonce: Option<Buffer>,
  pub parallelism: Option<u32>,
  pub tag_length: Option<u32>,
  pub memory: Option<u32>,
  pub passes: Option<u32>,
  pub secret: Option<Buffer>,
  pub associated_data: Option<Buffer>,
}

#[napi]
pub fn argon2_sync(algorithm: String, params: Argon2Params) -> Result<Buffer> {
  let algo = match algorithm.to_lowercase().as_str() {
    "argon2d" => Algorithm::Argon2d,
    "argon2i" => Algorithm::Argon2i,
    "argon2id" => Algorithm::Argon2id,
    _ => {
      return Err(Error::new(
        Status::InvalidArg,
        format!(
          "The argument 'algorithm' must be one of: 'argon2d', 'argon2i', 'argon2id'. Received '{}'",
          algorithm
        ),
      ));
    }
  };

  let msg = params.message.as_deref().unwrap_or(&[]);
  let nonce = params.nonce.as_deref().unwrap_or(&[]);

  if nonce.len() < 8 {
    return Err(Error::new(
      Status::InvalidArg,
      format!(
        "The value of \"parameters.nonce.byteLength\" is out of range. It must be >= 8 && <= 4294967295. Received {}",
        nonce.len()
      ),
    ));
  }

  let tag_len = params.tag_length.unwrap_or(64) as usize;
  if tag_len < 4 {
    return Err(Error::new(
      Status::InvalidArg,
      format!(
        "The value of \"parameters.tagLength\" is out of range. It must be >= 4 && <= 4294967295. Received {}",
        tag_len
      ),
    ));
  }

  let m_cost = params.memory.unwrap_or(8) as u64;
  let t_cost = params.passes.unwrap_or(3);
  let p_cost = params.parallelism.unwrap_or(1);

  if p_cost == 0 || p_cost > 16777215 {
    return Err(Error::new(
      Status::InvalidArg,
      format!(
        "The value of \"parameters.parallelism\" is out of range. It must be >= 1 && <= 16777215. Received {}",
        p_cost
      ),
    ));
  }

  let argon_params = Params::builder()
    .memory(argon2_rust::params::Memory::kib(m_cost))
    .passes(t_cost)
    .lanes(p_cost)
    .tag_len(argon2_rust::params::TagLen::bytes(tag_len as u64))
    .build()
    .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid Argon2 params: {}", e)))?;

  let argon2 = Argon2::new(algo, Version::V0x13, argon_params);

  let secret = params.secret.as_deref().unwrap_or(&[]);
  let ad = params.associated_data.as_deref().unwrap_or(&[]);

  let output = argon2
    .hash_with_ad(msg, nonce, secret, ad)
    .map_err(|e| Error::new(Status::GenericFailure, format!("Argon2 hash error: {}", e)))?;

  Ok(Buffer::from(output))
}

#[napi]
pub async fn argon2(algorithm: String, params: Argon2Params) -> Result<Buffer> {
  argon2_sync(algorithm, params)
}
