use napi::bindgen_prelude::*;
use napi_derive::napi;
use rand::Rng;
use uuid::Uuid;

#[napi(js_name = "randomBytes")]
pub fn random_bytes(size: u32) -> Result<Buffer> {
  let mut bytes = vec![0u8; size as usize];
  rand::thread_rng().fill(&mut bytes[..]);
  Ok(Buffer::from(bytes))
}

#[napi(js_name = "randomFillSync")]
pub fn random_fill_sync(
  mut buffer: Buffer,
  offset: Option<u32>,
  size: Option<u32>,
) -> Result<Buffer> {
  let buf_mut = buffer.as_mut();
  let start = offset.unwrap_or(0) as usize;
  let len = size.unwrap_or((buf_mut.len() - start) as u32) as usize;
  if start + len > buf_mut.len() {
    return Err(Error::from_reason("Range out of bounds for randomFillSync"));
  }
  rand::thread_rng().fill(&mut buf_mut[start..start + len]);
  Ok(buffer)
}

#[napi(js_name = "randomFill")]
pub fn random_fill(
  buffer: Buffer,
  offset: Option<u32>,
  size: Option<u32>,
) -> Result<Buffer> {
  random_fill_sync(buffer, offset, size)
}

#[napi(js_name = "randomInt")]
pub fn random_int(min_or_max: i64, max: Option<i64>) -> Result<i64> {
  let (min_val, max_val) = match max {
    Some(m) => (min_or_max, m),
    None => (0, min_or_max),
  };
  if min_val >= max_val {
    return Err(Error::from_reason("min must be strictly less than max"));
  }
  let val = rand::thread_rng().gen_range(min_val..max_val);
  Ok(val)
}

#[napi(js_name = "randomUUID")]
pub fn random_uuid() -> String {
  Uuid::new_v4().to_string()
}

#[napi(js_name = "randomUUIDv7")]
pub fn random_uuid_v7() -> String {
  Uuid::now_v7().to_string()
}
