use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::rand::{SecureRandom, SystemRandom};
use uuid::Uuid;

fn get_system_random() -> SystemRandom {
  SystemRandom::new()
}

#[napi]
pub fn random_bytes(size: u32) -> Result<Buffer> {
  let rng = get_system_random();
  let mut bytes = vec![0u8; size as usize];
  rng
    .fill(&mut bytes)
    .map_err(|_| Error::new(Status::GenericFailure, "Failed to generate random bytes"))?;
  Ok(Buffer::from(bytes))
}

#[napi]
pub fn random_fill_sync(
  #[napi(ts_arg_type = "Uint8Array")] mut buffer: Uint8Array,
  offset: Option<u32>,
  size: Option<u32>,
) -> Result<Uint8Array> {
  let start = offset.unwrap_or(0) as usize;
  let len = size.map(|s| s as usize).unwrap_or(buffer.len() - start);

  if start + len > buffer.len() {
    return Err(Error::new(
      Status::InvalidArg,
      "Offset and size out of buffer bounds",
    ));
  }

  let rng = get_system_random();
  let slice = unsafe { buffer.as_mut() };
  rng
    .fill(&mut slice[start..start + len])
    .map_err(|_| Error::new(Status::GenericFailure, "Failed to fill random bytes"))?;

  Ok(buffer)
}

#[napi]
pub fn random_int(min_or_max: i64, max: Option<i64>) -> Result<i64> {
  let (min, max_val) = match max {
    Some(m) => (min_or_max, m),
    None => (0, min_or_max),
  };

  if min >= max_val {
    return Err(Error::new(
      Status::InvalidArg,
      "min must be less than max",
    ));
  }

  let range = (max_val - min) as u64;
  let rng = get_system_random();
  let mut bytes = [0u8; 8];
  rng
    .fill(&mut bytes)
    .map_err(|_| Error::new(Status::GenericFailure, "Random generation failed"))?;

  let val = u64::from_le_bytes(bytes);
  let random_val = (val % range) as i64 + min;

  Ok(random_val)
}

#[napi]
pub fn random_uuid() -> String {
  Uuid::new_v4().to_string()
}
