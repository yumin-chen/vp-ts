use rand::RngCore;
use rand::rngs::OsRng;
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub fn random_bytes_sync(size: u32) -> Result<Buffer> {
  if size > (i32::MAX as u32) {
    return Err(Error::new(
      Status::InvalidArg,
      "ERR_OUT_OF_RANGE: Size must not be larger than 2**31 - 1",
    ));
  }
  let mut buf = vec![0u8; size as usize];
  OsRng.fill_bytes(&mut buf);
  Ok(Buffer::from(buf))
}

pub struct RandomBytesTask {
  size: u32,
}

#[napi]
impl Task for RandomBytesTask {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let mut buf = vec![0u8; self.size as usize];
    OsRng.fill_bytes(&mut buf);
    Ok(buf)
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(Buffer::from(output))
  }
}

#[napi]
pub fn random_bytes(size: u32) -> AsyncTask<RandomBytesTask> {
  AsyncTask::new(RandomBytesTask { size })
}

#[napi]
pub fn random_fill_sync(
  mut buffer: Buffer,
  offset: Option<u32>,
  size: Option<u32>,
) -> Result<Buffer> {
  let buf_ref = buffer.as_mut();
  let start = offset.unwrap_or(0) as usize;
  let len = size.unwrap_or_else(|| (buf_ref.len().saturating_sub(start)) as u32) as usize;

  if start + len > buf_ref.len() {
    return Err(Error::new(
      Status::InvalidArg,
      "ERR_OUT_OF_RANGE: Offset + size is out of buffer bounds",
    ));
  }

  OsRng.fill_bytes(&mut buf_ref[start..start + len]);
  Ok(buffer)
}

#[napi]
pub fn random_int_sync(min: i64, max: Option<i64>) -> Result<i64> {
  let (start, end) = match max {
    Some(m) => (min, m),
    None => (0, min),
  };
  if start >= end {
    return Err(Error::new(
      Status::InvalidArg,
      "ERR_OUT_OF_RANGE: min must be less than max",
    ));
  }
  use rand::Rng;
  let val = OsRng.gen_range(start..end);
  Ok(val)
}

#[napi]
pub fn random_uuid() -> String {
  let mut bytes = [0u8; 16];
  OsRng.fill_bytes(&mut bytes);
  // Set version 4 and variant
  bytes[6] = (bytes[6] & 0x0f) | 0x40;
  bytes[8] = (bytes[8] & 0x3f) | 0x80;
  format!(
    "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
    u32::from_be_bytes(bytes[0..4].try_into().unwrap()),
    u16::from_be_bytes(bytes[4..6].try_into().unwrap()),
    u16::from_be_bytes(bytes[6..8].try_into().unwrap()),
    u16::from_be_bytes(bytes[8..10].try_into().unwrap()),
    u64::from_be_bytes([0, 0, bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]])
  )
}

#[napi]
pub fn random_uuid_v7() -> String {
  let mut bytes = [0u8; 16];
  OsRng.fill_bytes(&mut bytes);
  let now_ms = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .unwrap_or_default()
    .as_millis() as u64;

  let ts_bytes = now_ms.to_be_bytes();
  bytes[0..6].copy_from_slice(&ts_bytes[2..8]);
  // Set version 7 and variant
  bytes[6] = (bytes[6] & 0x0f) | 0x70;
  bytes[8] = (bytes[8] & 0x3f) | 0x80;

  format!(
    "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
    u32::from_be_bytes(bytes[0..4].try_into().unwrap()),
    u16::from_be_bytes(bytes[4..6].try_into().unwrap()),
    u16::from_be_bytes(bytes[6..8].try_into().unwrap()),
    u16::from_be_bytes(bytes[8..10].try_into().unwrap()),
    u64::from_be_bytes([0, 0, bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]])
  )
}
