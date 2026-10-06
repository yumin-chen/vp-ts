use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::rand::{SecureRandom, SystemRandom};
use uuid::Uuid;

#[napi]
pub fn random_bytes(size: u32) -> Result<Buffer> {
  if size > 2147483647 {
    return Err(Error::new(
      Status::InvalidArg,
      "The value of \"size\" is out of range. It must be <= 2147483647.",
    ));
  }
  let rng = SystemRandom::new();
  let mut bytes = vec![0u8; size as usize];
  rng
    .fill(&mut bytes)
    .map_err(|_| Error::new(Status::GenericFailure, "Failed to generate random bytes"))?;
  Ok(Buffer::from(bytes))
}

#[napi]
pub fn random_fill_sync(
  mut buffer: Buffer,
  offset: Option<u32>,
  size: Option<u32>,
) -> Result<Buffer> {
  let start = offset.unwrap_or(0) as usize;
  let len = size.unwrap_or((buffer.len() - start) as u32) as usize;
  if start + len > buffer.len() {
    return Err(Error::new(Status::InvalidArg, "Offset and size out of bounds"));
  }

  let rng = SystemRandom::new();
  rng
    .fill(&mut buffer[start..start + len])
    .map_err(|_| Error::new(Status::GenericFailure, "Failed to fill random bytes"))?;
  Ok(buffer)
}

#[napi]
pub fn random_int(min_or_max: i64, max: Option<i64>) -> Result<i64> {
  let (min, max_val) = match max {
    Some(m) => (min_or_max, m),
    None => (0i64, min_or_max),
  };

  if min >= max_val {
    return Err(Error::new(Status::InvalidArg, "min must be less than max"));
  }

  let range = (max_val - min) as u64;
  let rng = SystemRandom::new();
  let mut bytes = [0u8; 8];
  rng
    .fill(&mut bytes)
    .map_err(|_| Error::new(Status::GenericFailure, "Failed to generate random int"))?;
  let val = u64::from_le_bytes(bytes);
  let res = (val % range) as i64 + min;
  Ok(res)
}

#[napi(js_name = "randomUUID")]
pub fn random_uuid() -> String {
  Uuid::new_v4().to_string()
}

#[napi(js_name = "randomUUIDv7")]
pub fn random_uuidv7() -> String {
  Uuid::now_v7().to_string()
}

#[napi]
pub fn get_random_values(mut buffer: Buffer) -> Result<Buffer> {
  let rng = SystemRandom::new();
  rng
    .fill(&mut buffer)
    .map_err(|_| Error::new(Status::GenericFailure, "Failed to fill random values"))?;
  Ok(buffer)
}
