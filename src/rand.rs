use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::rand::SecureRandom;

#[napi]
pub fn random_bytes(size: u32) -> Result<Buffer> {
  let rng = ring::rand::SystemRandom::new();
  let mut buf = vec![0u8; size as usize];
  rng
    .fill(&mut buf)
    .map_err(|_| Error::new(Status::GenericFailure, "Random generation failed"))?;
  Ok(Buffer::from(buf))
}

#[napi]
pub fn random_fill_sync(mut buffer: Buffer, offset: Option<u32>, size: Option<u32>) -> Result<Buffer> {
  let off = offset.unwrap_or(0) as usize;
  let len = size.unwrap_or((buffer.len() - off) as u32) as usize;

  if off + len > buffer.len() {
    return Err(Error::new(Status::InvalidArg, "Offset and size out of bounds"));
  }

  let rng = ring::rand::SystemRandom::new();
  rng
    .fill(&mut buffer[off..off + len])
    .map_err(|_| Error::new(Status::GenericFailure, "Random generation failed"))?;

  Ok(buffer)
}

#[napi]
pub fn random_int(min: i64, max: i64) -> Result<i64> {
  if min >= max {
    return Err(Error::new(Status::InvalidArg, "Min must be less than max"));
  }
  let range = (max - min) as u64;
  let max_valid = u64::MAX - (u64::MAX % range);
  let rng = ring::rand::SystemRandom::new();
  let mut bytes = [0u8; 8];

  loop {
    rng
      .fill(&mut bytes)
      .map_err(|_| Error::new(Status::GenericFailure, "Random generation failed"))?;
    let val = u64::from_le_bytes(bytes);
    if val < max_valid {
      return Ok(min + (val % range) as i64);
    }
  }
}

#[napi]
pub fn random_uuid() -> String {
  let rng = ring::rand::SystemRandom::new();
  let mut bytes = [0u8; 16];
  let _ = rng.fill(&mut bytes);
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
