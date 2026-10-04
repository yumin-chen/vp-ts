use napi::bindgen_prelude::*;
use napi_derive::napi;
use rand::{Rng, RngCore};
use uuid::Uuid;

#[napi]
pub fn random_bytes(size: u32) -> Result<Buffer> {
  let mut buf = vec![0u8; size as usize];
  rand::thread_rng().fill_bytes(&mut buf);
  Ok(Buffer::from(buf))
}

#[napi]
pub fn random_fill_sync(mut buffer: Buffer, offset: Option<u32>, size: Option<u32>) -> Buffer {
  let start = offset.unwrap_or(0) as usize;
  let len = size.unwrap_or((buffer.len() - start) as u32) as usize;
  let end = start + len;

  if start < buffer.len() && end <= buffer.len() {
    rand::thread_rng().fill_bytes(&mut buffer[start..end]);
  }
  buffer
}

#[napi]
pub fn random_int(min: i64, max: i64) -> Result<i64> {
  if min >= max {
    return Err(Error::new(
      Status::InvalidArg,
      "min must be strictly less than max",
    ));
  }
  let val = rand::thread_rng().gen_range(min..max);
  Ok(val)
}

#[napi]
pub fn random_uuid() -> String {
  Uuid::new_v4().to_string()
}

#[napi]
pub fn random_uuidv7() -> String {
  Uuid::now_v7().to_string()
}
