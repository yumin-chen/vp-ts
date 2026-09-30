use napi::bindgen_prelude::*;
use napi_derive::napi;
use svix_ksuid::{Ksuid, KsuidLike};

#[napi]
pub fn new_ksuid() -> String {
  Ksuid::new(None, None).to_string()
}

#[napi]
pub fn ksuid_from_base62(base62: String) -> Result<String> {
  let ksuid = Ksuid::from_base62(&base62)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid KSUID base62 string: {}", e)))?;
  Ok(ksuid.to_string())
}

#[napi]
pub fn ksuid_to_bytes(base62: String) -> Result<Uint8Array> {
  let ksuid = Ksuid::from_base62(&base62)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid KSUID base62 string: {}", e)))?;
  Ok(Uint8Array::from(ksuid.bytes().to_vec()))
}

#[napi]
pub fn ksuid_timestamp_seconds(base62: String) -> Result<i64> {
  let ksuid = Ksuid::from_base62(&base62)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid KSUID base62 string: {}", e)))?;
  Ok(ksuid.timestamp_seconds())
}
