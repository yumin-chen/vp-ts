use napi::bindgen_prelude::*;
use napi_derive::napi;
use rand::{RngCore, Rng};
use uuid::Uuid;

#[napi]
pub fn random_bytes(size: u32) -> Buffer {
  let mut bytes = vec![0u8; size as usize];
  rand::thread_rng().fill_bytes(&mut bytes);
  Buffer::from(bytes)
}

#[napi]
pub fn random_fill_sync(mut buffer: Buffer) -> Buffer {
  rand::thread_rng().fill_bytes(&mut buffer);
  buffer
}

#[napi]
pub fn random_int(min: i32, max: i32) -> i32 {
  if min >= max {
    min
  } else {
    rand::thread_rng().gen_range(min..max)
  }
}

#[napi]
pub fn random_uuid() -> String {
  Uuid::new_v4().to_string()
}
