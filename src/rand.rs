#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::rand::SecureRandom;
use uuid::Uuid;

#[napi]
pub fn random_bytes(size: u32) -> Result<Buffer> {
  let mut bytes = vec![0u8; size as usize];
  ring::rand::SystemRandom::new()
    .fill(&mut bytes)
    .map_err(|_| Error::new(Status::GenericFailure, "Failed to generate random bytes"))?;
  Ok(Buffer::from(bytes))
}

#[napi]
pub fn random_fill_sync(
  mut buffer: Uint8Array,
  offset: Option<u32>,
  size: Option<u32>,
) -> Result<Uint8Array> {
  let buf_slice = unsafe { buffer.as_mut() };
  let start = offset.unwrap_or(0) as usize;
  let len = size.unwrap_or_else(|| (buf_slice.len() - start) as u32) as usize;

  if start + len > buf_slice.len() {
    return Err(Error::new(Status::InvalidArg, "Range out of bounds"));
  }

  ring::rand::SystemRandom::new()
    .fill(&mut buf_slice[start..start + len])
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
    return Err(Error::new(Status::InvalidArg, "min must be less than max"));
  }

  let range = (max_val - min) as u64;
  let mut bytes = [0u8; 8];
  ring::rand::SystemRandom::new()
    .fill(&mut bytes)
    .map_err(|_| Error::new(Status::GenericFailure, "Failed to generate random int"))?;

  let num = u64::from_le_bytes(bytes);
  let val = min + (num % range) as i64;
  Ok(val)
}

#[napi(js_name = "randomUUID")]
pub fn random_uuid() -> String {
  Uuid::new_v4().to_string()
}

#[napi(js_name = "randomUUIDv7")]
pub fn random_uuidv7() -> String {
  Uuid::now_v7().to_string()
}
