use argon2::{Algorithm, Argon2, Params, Version};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(object)]
pub struct Argon2Parameters {
  pub message: Either<Buffer, String>,
  pub nonce: Either<Buffer, String>,
  pub parallelism: u32,
  pub tag_length: u32,
  pub memory: u32,
  pub passes: u32,
  pub secret: Option<Either<Buffer, String>>,
  pub associated_data: Option<Either<Buffer, String>>,
}

#[napi(js_name = "argon2Sync")]
pub fn argon2_sync(algorithm: String, parameters: Argon2Parameters) -> Result<Buffer> {
  let alg = match algorithm.to_lowercase().as_str() {
    "argon2i" => Algorithm::Argon2i,
    "argon2d" => Algorithm::Argon2d,
    "argon2id" => Algorithm::Argon2id,
    other => return Err(Error::from_reason(format!("Unsupported argon2 algorithm: {other}"))),
  };

  let message_bytes = match parameters.message {
    Either::A(b) => b.as_ref().to_vec(),
    Either::B(s) => s.as_bytes().to_vec(),
  };

  let nonce_bytes = match parameters.nonce {
    Either::A(b) => b.as_ref().to_vec(),
    Either::B(s) => s.as_bytes().to_vec(),
  };

  if nonce_bytes.len() < 8 {
    return Err(Error::from_reason("nonce must be at least 8 bytes long"));
  }

  let params = Params::new(
    parameters.memory,
    parameters.passes,
    parameters.parallelism,
    Some(parameters.tag_length as usize),
  )
  .map_err(|e| Error::from_reason(format!("Invalid Argon2 params: {e}")))?;

  let argon2 = Argon2::new(alg, Version::V0x13, params);

  let mut output = vec![0u8; parameters.tag_length as usize];
  argon2
    .hash_password_into(&message_bytes, &nonce_bytes, &mut output)
    .map_err(|e| Error::from_reason(format!("Argon2 hashing error: {e}")))?;

  Ok(Buffer::from(output))
}

#[napi(js_name = "argon2")]
pub fn argon2(algorithm: String, parameters: Argon2Parameters) -> Result<Buffer> {
  argon2_sync(algorithm, parameters)
}
