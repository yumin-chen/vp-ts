use napi::bindgen_prelude::*;
use napi_derive::napi;
use svix_ksuid::{Ksuid as SvixKsuid, KsuidLike, KsuidMs as SvixKsuidMs};

fn parse_payload(payload: Option<Uint8Array>) -> Result<Option<Vec<u8>>> {
  match payload {
    Some(p) => Ok(Some(p.as_ref().to_vec())),
    None => Ok(None),
  }
}

fn parse_timestamp(timestamp: Option<f64>) -> Result<Option<jiff::Timestamp>> {
  match timestamp {
    Some(ts) => {
      let sec = if ts.abs() > 1e11 {
        (ts / 1000.0) as i64
      } else {
        ts as i64
      };
      jiff::Timestamp::from_second(sec)
        .map(Some)
        .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))
    }
    None => Ok(None),
  }
}

#[napi]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ksuid {
  inner: SvixKsuid,
}

#[napi]
impl Ksuid {
  #[napi(getter, js_name = "PAYLOAD_BYTES")]
  pub fn payload_bytes_const() -> u32 {
    16
  }

  #[napi(getter, js_name = "BYTES")]
  pub fn bytes_len_const() -> u32 {
    20
  }

  #[napi]
  pub fn now(payload: Option<Uint8Array>) -> Result<Self> {
    let p = parse_payload(payload)?;
    Ok(Self {
      inner: SvixKsuid::now(p.as_deref()),
    })
  }

  #[napi]
  pub fn new(timestamp: Option<f64>, payload: Option<Uint8Array>) -> Result<Self> {
    let ts = parse_timestamp(timestamp)?;
    let p = parse_payload(payload)?;
    Ok(Self {
      inner: SvixKsuid::new(ts, p.as_deref()),
    })
  }

  #[napi(js_name = "fromBase62")]
  pub fn from_base62(base62: String) -> Result<Self> {
    let inner = SvixKsuid::from_base62(&base62)
      .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
    Ok(Self { inner })
  }

  #[napi(js_name = "from_base62")]
  pub fn from_base62_snake(base62: String) -> Result<Self> {
    Self::from_base62(base62)
  }

  #[napi(js_name = "fromStr")]
  pub fn from_str_js(base62: String) -> Result<Self> {
    Self::from_base62(base62)
  }

  #[napi(js_name = "from_str")]
  pub fn from_str_snake(base62: String) -> Result<Self> {
    Self::from_base62(base62)
  }

  #[napi(js_name = "fromBytes")]
  pub fn from_bytes(bytes: Uint8Array) -> Result<Self> {
    let slice = bytes.as_ref();
    if slice.len() != 20 {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Bytes must be exactly 20 bytes, got {}", slice.len()),
      ));
    }
    let mut arr = [0u8; 20];
    arr.copy_from_slice(slice);
    Ok(Self {
      inner: SvixKsuid::from_bytes(arr),
    })
  }

  #[napi(js_name = "from_bytes")]
  pub fn from_bytes_snake(bytes: Uint8Array) -> Result<Self> {
    Self::from_bytes(bytes)
  }

  #[napi(js_name = "fromSeconds")]
  pub fn from_seconds(seconds: Option<f64>, payload: Option<Uint8Array>) -> Result<Self> {
    let sec = seconds.map(|s| s as i64);
    let p = parse_payload(payload)?;
    Ok(Self {
      inner: SvixKsuid::from_seconds(sec, p.as_deref()),
    })
  }

  #[napi(js_name = "from_seconds")]
  pub fn from_seconds_snake(seconds: Option<f64>, payload: Option<Uint8Array>) -> Result<Self> {
    Self::from_seconds(seconds, payload)
  }

  #[napi(js_name = "toBase62")]
  pub fn to_base62(&self) -> String {
    self.inner.to_base62()
  }

  #[napi(js_name = "to_base62")]
  pub fn to_base62_snake(&self) -> String {
    self.inner.to_base62()
  }

  #[napi(js_name = "toString")]
  pub fn to_string_js(&self) -> String {
    self.inner.to_string()
  }

  #[napi(js_name = "to_string")]
  pub fn to_string_snake(&self) -> String {
    self.inner.to_string()
  }

  #[napi]
  pub fn bytes(&self) -> Buffer {
    Buffer::from(&self.inner.bytes()[..])
  }

  #[napi]
  pub fn payload(&self) -> Buffer {
    Buffer::from(self.inner.payload())
  }

  #[napi(js_name = "timestampSeconds")]
  pub fn timestamp_seconds(&self) -> f64 {
    self.inner.timestamp_seconds() as f64
  }

  #[napi(js_name = "timestamp_seconds")]
  pub fn timestamp_seconds_snake(&self) -> f64 {
    self.inner.timestamp_seconds() as f64
  }

  #[napi]
  pub fn timestamp(&self) -> f64 {
    (self.inner.timestamp_seconds() as f64) * 1000.0
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

  #[napi]
  pub fn gt(&self, other: &Ksuid) -> bool {
    self.inner > other.inner
  }

  #[napi]
  pub fn gte(&self, other: &Ksuid) -> bool {
    self.inner >= other.inner
  }

  #[napi]
  pub fn lt(&self, other: &Ksuid) -> bool {
    self.inner < other.inner
  }

  #[napi]
  pub fn lte(&self, other: &Ksuid) -> bool {
    self.inner <= other.inner
  }
}

#[napi]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct KsuidMs {
  inner: SvixKsuidMs,
}

#[napi]
impl KsuidMs {
  #[napi(getter, js_name = "PAYLOAD_BYTES")]
  pub fn payload_bytes_const() -> u32 {
    15
  }

  #[napi(getter, js_name = "BYTES")]
  pub fn bytes_len_const() -> u32 {
    20
  }

  #[napi]
  pub fn now(payload: Option<Uint8Array>) -> Result<Self> {
    let p = parse_payload(payload)?;
    Ok(Self {
      inner: SvixKsuidMs::now(p.as_deref()),
    })
  }

  #[napi]
  pub fn new(timestamp: Option<f64>, payload: Option<Uint8Array>) -> Result<Self> {
    let ts = parse_timestamp(timestamp)?;
    let p = parse_payload(payload)?;
    Ok(Self {
      inner: SvixKsuidMs::new(ts, p.as_deref()),
    })
  }

  #[napi(js_name = "fromBase62")]
  pub fn from_base62(base62: String) -> Result<Self> {
    let inner = SvixKsuidMs::from_base62(&base62)
      .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
    Ok(Self { inner })
  }

  #[napi(js_name = "from_base62")]
  pub fn from_base62_snake(base62: String) -> Result<Self> {
    Self::from_base62(base62)
  }

  #[napi(js_name = "fromStr")]
  pub fn from_str_js(base62: String) -> Result<Self> {
    Self::from_base62(base62)
  }

  #[napi(js_name = "from_str")]
  pub fn from_str_snake(base62: String) -> Result<Self> {
    Self::from_base62(base62)
  }

  #[napi(js_name = "fromBytes")]
  pub fn from_bytes(bytes: Uint8Array) -> Result<Self> {
    let slice = bytes.as_ref();
    if slice.len() != 20 {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Bytes must be exactly 20 bytes, got {}", slice.len()),
      ));
    }
    let mut arr = [0u8; 20];
    arr.copy_from_slice(slice);
    Ok(Self {
      inner: SvixKsuidMs::from_bytes(arr),
    })
  }

  #[napi(js_name = "from_bytes")]
  pub fn from_bytes_snake(bytes: Uint8Array) -> Result<Self> {
    Self::from_bytes(bytes)
  }

  #[napi(js_name = "fromMillis")]
  pub fn from_millis(millis: Option<f64>, payload: Option<Uint8Array>) -> Result<Self> {
    let ms = millis.map(|m| m as i64);
    let p = parse_payload(payload)?;
    Ok(Self {
      inner: SvixKsuidMs::from_millis(ms, p.as_deref()),
    })
  }

  #[napi(js_name = "from_millis")]
  pub fn from_millis_snake(millis: Option<f64>, payload: Option<Uint8Array>) -> Result<Self> {
    Self::from_millis(millis, payload)
  }

  #[napi(js_name = "fromSeconds")]
  pub fn from_seconds(seconds: Option<f64>, payload: Option<Uint8Array>) -> Result<Self> {
    let sec = seconds.map(|s| (s * 1000.0) as i64);
    let p = parse_payload(payload)?;
    Ok(Self {
      inner: SvixKsuidMs::from_millis(sec, p.as_deref()),
    })
  }

  #[napi(js_name = "from_seconds")]
  pub fn from_seconds_snake(seconds: Option<f64>, payload: Option<Uint8Array>) -> Result<Self> {
    Self::from_seconds(seconds, payload)
  }

  #[napi(js_name = "toBase62")]
  pub fn to_base62(&self) -> String {
    self.inner.to_base62()
  }

  #[napi(js_name = "to_base62")]
  pub fn to_base62_snake(&self) -> String {
    self.inner.to_base62()
  }

  #[napi(js_name = "toString")]
  pub fn to_string_js(&self) -> String {
    self.inner.to_string()
  }

  #[napi(js_name = "to_string")]
  pub fn to_string_snake(&self) -> String {
    self.inner.to_string()
  }

  #[napi]
  pub fn bytes(&self) -> Buffer {
    Buffer::from(&self.inner.bytes()[..])
  }

  #[napi]
  pub fn payload(&self) -> Buffer {
    Buffer::from(self.inner.payload())
  }

  #[napi(js_name = "timestampSeconds")]
  pub fn timestamp_seconds(&self) -> f64 {
    self.inner.timestamp_seconds() as f64
  }

  #[napi(js_name = "timestamp_seconds")]
  pub fn timestamp_seconds_snake(&self) -> f64 {
    self.inner.timestamp_seconds() as f64
  }

  #[napi(js_name = "timestampMillis")]
  pub fn timestamp_millis(&self) -> f64 {
    (self.inner.timestamp_seconds() as f64) * 1000.0
  }

  #[napi(js_name = "timestamp_millis")]
  pub fn timestamp_millis_snake(&self) -> f64 {
    self.timestamp_millis()
  }

  #[napi]
  pub fn timestamp(&self) -> f64 {
    self.timestamp_millis()
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

  #[napi]
  pub fn gt(&self, other: &KsuidMs) -> bool {
    self.inner > other.inner
  }

  #[napi]
  pub fn gte(&self, other: &KsuidMs) -> bool {
    self.inner >= other.inner
  }

  #[napi]
  pub fn lt(&self, other: &KsuidMs) -> bool {
    self.inner < other.inner
  }

  #[napi]
  pub fn lte(&self, other: &KsuidMs) -> bool {
    self.inner <= other.inner
  }
}
