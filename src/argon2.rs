use argon2::{
  Algorithm as Argon2Algorithm, Argon2, ParamsBuilder, Version as Argon2Version,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(object)]
#[derive(Default)]
pub struct Argon2Options {
  pub memory_cost: Option<u32>,
  pub time_cost: Option<u32>,
  pub parallelism: Option<u32>,
  pub tag_length: Option<u32>,
  pub secret: Option<Buffer>,
  pub associated_data: Option<Buffer>,
}

#[napi]
pub fn argon2_sync(
  password: Either<String, Buffer>,
  salt: Either<String, Buffer>,
  options: Option<Argon2Options>,
) -> Result<Buffer> {
  let opts = options.unwrap_or_default();
  let pass_bytes = match password {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.to_vec(),
  };
  let salt_bytes = match salt {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.to_vec(),
  };

  let mut builder = ParamsBuilder::new();
  if let Some(m) = opts.memory_cost {
    builder.m_cost(m);
  }
  if let Some(t) = opts.time_cost {
    builder.t_cost(t);
  }
  if let Some(p) = opts.parallelism {
    builder.p_cost(p);
  }
  let tag_len = opts.tag_length.unwrap_or(32) as usize;
  if let Some(tl) = opts.tag_length {
    builder.output_len(tl as usize);
  }

  let params = builder.build().map_err(|e| Error::from_reason(e.to_string()))?;
  let secret_bytes = opts.secret.as_ref().map(|b| b.as_ref()).unwrap_or(&[]);

  let argon2 = if secret_bytes.is_empty() {
    Argon2::new(Argon2Algorithm::Argon2id, Argon2Version::V0x13, params)
  } else {
    Argon2::new_with_secret(secret_bytes, Argon2Algorithm::Argon2id, Argon2Version::V0x13, params)
      .map_err(|e| Error::from_reason(e.to_string()))?
  };

  let mut output = vec![0u8; tag_len];
  argon2
    .hash_password_into(&pass_bytes, &salt_bytes, &mut output)
    .map_err(|e| Error::from_reason(e.to_string()))?;

  Ok(Buffer::from(output))
}

#[napi]
pub fn argon2(
  password: Either<String, Buffer>,
  salt: Either<String, Buffer>,
  options: Option<Argon2Options>,
) -> Result<Buffer> {
  argon2_sync(password, salt, options)
}
