use napi::bindgen_prelude::*;
use napi_derive::napi;
use num_bigint_dig::prime::probably_prime;
use num_bigint_dig::BigUint;
use ring::rand::SecureRandom;

use crate::hmac::decode_input;

#[napi]
pub fn random_bytes(size: u32) -> Buffer {
  let rng = ring::rand::SystemRandom::new();
  let mut bytes = vec![0u8; size as usize];
  let _ = rng.fill(&mut bytes);
  Buffer::from(bytes)
}

#[napi(ts_args_type = "buffer: Buffer, offset?: number, size?: number", ts_return_type = "Buffer")]
pub fn random_fill_sync(mut buffer: Buffer, offset: Option<u32>, size: Option<u32>) -> Buffer {
  let off = offset.unwrap_or(0) as usize;
  let len = buffer.len();
  let sz = size.unwrap_or((len.saturating_sub(off)) as u32) as usize;

  if off + sz <= len {
    let rng = ring::rand::SystemRandom::new();
    let _ = rng.fill(&mut buffer.as_mut()[off..off + sz]);
  }

  buffer
}

#[napi(ts_args_type = "buffer: Buffer", ts_return_type = "Buffer")]
pub fn random_fill(buffer: Buffer) -> Buffer {
  random_fill_sync(buffer, None, None)
}

#[napi]
pub fn random_int(a: u32, b: Option<u32>) -> u32 {
  let (min, max) = match b {
    Some(val) => (a, val),
    None => (0, a),
  };

  if min >= max {
    return min;
  }

  let range = max - min;
  let rng = ring::rand::SystemRandom::new();

  // Rejection sampling to avoid modulo bias
  let zone = u32::MAX - (u32::MAX % range);
  loop {
    let mut buf = [0u8; 4];
    let _ = rng.fill(&mut buf);
    let val = u32::from_be_bytes(buf);
    if val < zone {
      return min + (val % range);
    }
  }
}

#[napi]
pub fn random_uuid() -> String {
  let rng = ring::rand::SystemRandom::new();
  let mut bytes = [0u8; 16];
  let _ = rng.fill(&mut bytes);
  bytes[6] = (bytes[6] & 0x0f) | 0x40; // Version 4
  bytes[8] = (bytes[8] & 0x3f) | 0x80; // Variant 10xx
  format!(
    "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
    u32::from_be_bytes(bytes[0..4].try_into().unwrap()),
    u16::from_be_bytes(bytes[4..6].try_into().unwrap()),
    u16::from_be_bytes(bytes[6..8].try_into().unwrap()),
    u16::from_be_bytes(bytes[8..10].try_into().unwrap()),
    u64::from_be_bytes({
      let mut b = [0u8; 8];
      b[2..8].copy_from_slice(&bytes[10..16]);
      b
    }) & 0x0000FFFFFFFFFFFF
  )
}

#[napi]
pub fn random_uuid_v7() -> String {
  let rng = ring::rand::SystemRandom::new();
  let mut random_bytes = [0u8; 10];
  let _ = rng.fill(&mut random_bytes);

  let now_ms = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .unwrap_or_default()
    .as_millis() as u64;

  let mut uuid_bytes = [0u8; 16];
  uuid_bytes[0] = ((now_ms >> 40) & 0xFF) as u8;
  uuid_bytes[1] = ((now_ms >> 32) & 0xFF) as u8;
  uuid_bytes[2] = ((now_ms >> 24) & 0xFF) as u8;
  uuid_bytes[3] = ((now_ms >> 16) & 0xFF) as u8;
  uuid_bytes[4] = ((now_ms >> 8) & 0xFF) as u8;
  uuid_bytes[5] = (now_ms & 0xFF) as u8;

  // Version 7: 0x70
  uuid_bytes[6] = 0x70 | (random_bytes[0] & 0x0F);
  uuid_bytes[7] = random_bytes[1];

  // Variant 10xx: 0x80
  uuid_bytes[8] = 0x80 | (random_bytes[2] & 0x3F);
  uuid_bytes[9] = random_bytes[3];
  uuid_bytes[10] = random_bytes[4];
  uuid_bytes[11] = random_bytes[5];
  uuid_bytes[12] = random_bytes[6];
  uuid_bytes[13] = random_bytes[7];
  uuid_bytes[14] = random_bytes[8];
  uuid_bytes[15] = random_bytes[9];

  format!(
    "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
    u32::from_be_bytes(uuid_bytes[0..4].try_into().unwrap()),
    u16::from_be_bytes(uuid_bytes[4..6].try_into().unwrap()),
    u16::from_be_bytes(uuid_bytes[6..8].try_into().unwrap()),
    u16::from_be_bytes(uuid_bytes[8..10].try_into().unwrap()),
    u64::from_be_bytes({
      let mut b = [0u8; 8];
      b[2..8].copy_from_slice(&uuid_bytes[10..16]);
      b
    }) & 0x0000FFFFFFFFFFFF
  )
}

#[napi(ts_args_type = "candidate: string | Buffer")]
pub fn check_prime_sync(candidate: Either<String, Buffer>) -> bool {
  let bytes = decode_input(&candidate, None);
  if bytes.is_empty() {
    return false;
  }
  let n = BigUint::from_bytes_be(&bytes);
  probably_prime(&n, 20)
}

#[napi(ts_args_type = "candidate: string | Buffer")]
pub fn check_prime(candidate: Either<String, Buffer>) -> bool {
  check_prime_sync(candidate)
}

#[napi(ts_return_type = "Buffer")]
pub fn generate_prime_sync(size: u32) -> Buffer {
  let byte_len = (size as usize + 7) / 8;
  let rng = ring::rand::SystemRandom::new();
  loop {
    let mut bytes = vec![0u8; byte_len];
    let _ = rng.fill(&mut bytes);
    if let Some(first) = bytes.first_mut() {
      *first |= 0x80; // High bit set
    }
    if let Some(last) = bytes.last_mut() {
      *last |= 0x01; // Odd number
    }
    let n = BigUint::from_bytes_be(&bytes);
    if probably_prime(&n, 15) {
      return Buffer::from(bytes);
    }
  }
}

#[napi(ts_return_type = "Buffer")]
pub fn generate_prime(size: u32) -> Buffer {
  generate_prime_sync(size)
}
