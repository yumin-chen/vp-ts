use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::str::FromStr;
use svix_ksuid::KsuidLike;

#[napi]
pub struct Ksuid {
  inner: svix_ksuid::Ksuid,
}

#[napi]
impl Ksuid {
  #[napi(constructor)]
  pub fn new(timestamp_seconds: Option<i64>, payload: Option<Buffer>) -> Self {
    let payload_ref = payload.as_deref();
    let inner = svix_ksuid::Ksuid::from_seconds(timestamp_seconds, payload_ref);
    Self { inner }
  }

  #[napi(factory)]
  pub fn now() -> Self {
    Self {
      inner: svix_ksuid::Ksuid::now(None),
    }
  }

  #[napi(factory)]
  pub fn from_seconds(timestamp_seconds: Option<i64>, payload: Option<Buffer>) -> Self {
    let payload_ref = payload.as_deref();
    let inner = svix_ksuid::Ksuid::from_seconds(timestamp_seconds, payload_ref);
    Self { inner }
  }

  #[napi(factory)]
  pub fn from_base62(base62: String) -> Result<Self> {
    let inner = svix_ksuid::Ksuid::from_base62(&base62)
      .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid base62 Ksuid: {e}")))?;
    Ok(Self { inner })
  }

  #[napi(factory)]
  pub fn from_str(str: String) -> Result<Self> {
    let inner = svix_ksuid::Ksuid::from_str(&str)
      .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid Ksuid string: {e}")))?;
    Ok(Self { inner })
  }

  #[napi(factory)]
  pub fn from_bytes(bytes: Buffer) -> Result<Self> {
    if bytes.len() != 20 {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Ksuid bytes length must be 20, got {}", bytes.len()),
      ));
    }
    let mut arr = [0u8; 20];
    arr.copy_from_slice(&bytes);
    let inner = svix_ksuid::Ksuid::from_bytes(arr);
    Ok(Self { inner })
  }

  #[napi]
  pub fn to_base62(&self) -> String {
    self.inner.to_base62()
  }

  #[napi]
  pub fn to_string(&self) -> String {
    self.inner.to_string()
  }

  #[napi]
  pub fn bytes(&self) -> Buffer {
    Buffer::from(self.inner.bytes().as_ref())
  }

  #[napi]
  pub fn payload(&self) -> Buffer {
    Buffer::from(self.inner.payload())
  }

  #[napi]
  pub fn timestamp_seconds(&self) -> i64 {
    self.inner.timestamp_seconds() as i64
  }

  #[napi]
  pub fn compare(&self, other: &Ksuid) -> i32 {
    match self.inner.cmp(&other.inner) {
      std::cmp::Ordering::Less => -1,
      std::cmp::Ordering::Equal => 0,
      std::cmp::Ordering::Greater => 1,
    }
  }

  #[napi]
  pub fn equals(&self, other: &Ksuid) -> bool {
    self.inner == other.inner
  }
}

#[napi]
pub struct KsuidMs {
  inner: svix_ksuid::KsuidMs,
}

#[napi]
impl KsuidMs {
  #[napi(constructor)]
  pub fn new(timestamp_ms: Option<i64>, payload: Option<Buffer>) -> Self {
    let payload_ref = payload.as_deref();
    let inner = svix_ksuid::KsuidMs::from_millis(timestamp_ms, payload_ref);
    Self { inner }
  }

  #[napi(factory)]
  pub fn now() -> Self {
    Self {
      inner: svix_ksuid::KsuidMs::now(None),
    }
  }

  #[napi(factory)]
  pub fn from_millis(timestamp_ms: Option<i64>, payload: Option<Buffer>) -> Self {
    let payload_ref = payload.as_deref();
    let inner = svix_ksuid::KsuidMs::from_millis(timestamp_ms, payload_ref);
    Self { inner }
  }

  #[napi(factory)]
  pub fn from_base62(base62: String) -> Result<Self> {
    let inner = svix_ksuid::KsuidMs::from_base62(&base62)
      .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid base62 KsuidMs: {e}")))?;
    Ok(Self { inner })
  }

  #[napi(factory)]
  pub fn from_str(str: String) -> Result<Self> {
    let inner = svix_ksuid::KsuidMs::from_str(&str)
      .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid KsuidMs string: {e}")))?;
    Ok(Self { inner })
  }

  #[napi(factory)]
  pub fn from_bytes(bytes: Buffer) -> Result<Self> {
    if bytes.len() != 20 {
      return Err(Error::new(
        Status::InvalidArg,
        format!("KsuidMs bytes length must be 20, got {}", bytes.len()),
      ));
    }
    let mut arr = [0u8; 20];
    arr.copy_from_slice(&bytes);
    let inner = svix_ksuid::KsuidMs::from_bytes(arr);
    Ok(Self { inner })
  }

  #[napi]
  pub fn to_base62(&self) -> String {
    self.inner.to_base62()
  }

  #[napi]
  pub fn to_string(&self) -> String {
    self.inner.to_string()
  }

  #[napi]
  pub fn bytes(&self) -> Buffer {
    Buffer::from(self.inner.bytes().as_ref())
  }

  #[napi]
  pub fn payload(&self) -> Buffer {
    Buffer::from(self.inner.payload())
  }

  #[napi]
  pub fn timestamp_ms(&self) -> i64 {
    self.inner.timestamp_millis() as i64
  }

  #[napi]
  pub fn compare(&self, other: &KsuidMs) -> i32 {
    match self.inner.cmp(&other.inner) {
      std::cmp::Ordering::Less => -1,
      std::cmp::Ordering::Equal => 0,
      std::cmp::Ordering::Greater => 1,
    }
  }

  #[napi]
  pub fn equals(&self, other: &KsuidMs) -> bool {
    self.inner == other.inner
  }
}
