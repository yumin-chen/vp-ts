use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::str::FromStr;
use svix_ksuid::{Ksuid as RawKsuid, KsuidLike, KsuidMs as RawKsuidMs};

const KSUID_BYTES: usize = 20;
const KSUID_PAYLOAD_BYTES: usize = <RawKsuid as KsuidLike>::PAYLOAD_BYTES;
const KSUID_MS_PAYLOAD_BYTES: usize = <RawKsuidMs as KsuidLike>::PAYLOAD_BYTES;

fn parse_payload(payload: Option<Uint8Array>, expected_len: usize) -> napi::Result<Option<Vec<u8>>> {
  match payload {
    Some(p) => {
      let slice = p.as_ref();
      if slice.len() != expected_len {
        return Err(napi::Error::from_reason(format!(
          "Payload must be {} bytes",
          expected_len
        )));
      }
      Ok(Some(slice.to_vec()))
    }
    None => Ok(None),
  }
}

#[napi(js_name = "Ksuid")]
#[derive(Clone, Copy)]
pub struct JsKsuid {
  inner: RawKsuid,
}

#[napi]
impl JsKsuid {
  /// Create a new Ksuid with current timestamp and optional 16-byte payload.
  #[napi(factory)]
  pub fn now(payload: Option<Uint8Array>) -> napi::Result<JsKsuid> {
    let p = parse_payload(payload, KSUID_PAYLOAD_BYTES)?;
    Ok(JsKsuid {
      inner: RawKsuid::now(p.as_deref()),
    })
  }

  /// Create a Ksuid with optional timestamp (in seconds since epoch) and optional 16-byte payload.
  #[napi(factory)]
  pub fn new(timestamp_seconds: Option<f64>, payload: Option<Uint8Array>) -> napi::Result<JsKsuid> {
    let p = parse_payload(payload, KSUID_PAYLOAD_BYTES)?;
    let secs = timestamp_seconds.map(|s| s as i64);
    Ok(JsKsuid {
      inner: RawKsuid::from_seconds(secs, p.as_deref()),
    })
  }

  /// Create a Ksuid from seconds timestamp and optional payload.
  #[napi(factory)]
  pub fn from_seconds(seconds: Option<f64>, payload: Option<Uint8Array>) -> napi::Result<JsKsuid> {
    let p = parse_payload(payload, KSUID_PAYLOAD_BYTES)?;
    let secs = seconds.map(|s| s as i64);
    Ok(JsKsuid {
      inner: RawKsuid::from_seconds(secs, p.as_deref()),
    })
  }

  /// Parse a Ksuid from a base62 string.
  #[napi(factory)]
  pub fn from_base62(base62: String) -> napi::Result<JsKsuid> {
    RawKsuid::from_base62(&base62)
      .map(|inner| JsKsuid { inner })
      .map_err(|e| napi::Error::from_reason(e.to_string()))
  }

  /// Parse a Ksuid from string (implements FromStr).
  #[napi(factory)]
  pub fn from_str(base62: String) -> napi::Result<JsKsuid> {
    RawKsuid::from_str(&base62)
      .map(|inner| JsKsuid { inner })
      .map_err(|e| napi::Error::from_reason(e.to_string()))
  }

  /// Create a Ksuid from 20 raw bytes.
  #[napi(factory)]
  pub fn from_bytes(bytes: Uint8Array) -> napi::Result<JsKsuid> {
    let slice = bytes.as_ref();
    if slice.len() != KSUID_BYTES {
      return Err(napi::Error::from_reason(format!(
        "Ksuid bytes must be {} bytes",
        KSUID_BYTES
      )));
    }
    let mut arr = [0u8; KSUID_BYTES];
    arr.copy_from_slice(slice);
    Ok(JsKsuid {
      inner: RawKsuid::from_bytes(arr),
    })
  }

  /// Returns the base62 string representation.
  #[napi]
  pub fn to_base62(&self) -> String {
    self.inner.to_base62()
  }

  /// Returns string representation.
  #[napi]
  pub fn to_string(&self) -> String {
    self.inner.to_string()
  }

  /// Returns the 20 bytes of the Ksuid.
  #[napi]
  pub fn bytes(&self) -> Uint8Array {
    Uint8Array::new(self.inner.bytes().to_vec())
  }

  /// Returns the 16 bytes of payload.
  #[napi]
  pub fn payload(&self) -> Uint8Array {
    Uint8Array::new(self.inner.payload().to_vec())
  }

  /// Returns the timestamp in seconds since UNIX epoch.
  #[napi]
  pub fn timestamp_seconds(&self) -> f64 {
    self.inner.timestamp_seconds() as f64
  }

  /// Compare this Ksuid with another Ksuid (-1, 0, 1).
  #[napi]
  pub fn compare(&self, other: &JsKsuid) -> i32 {
    match self.inner.cmp(&other.inner) {
      std::cmp::Ordering::Less => -1,
      std::cmp::Ordering::Equal => 0,
      std::cmp::Ordering::Greater => 1,
    }
  }

  /// Check if equal to another Ksuid.
  #[napi]
  pub fn equals(&self, other: &JsKsuid) -> bool {
    self.inner == other.inner
  }
}

#[napi(js_name = "KsuidMs")]
#[derive(Clone, Copy)]
pub struct JsKsuidMs {
  inner: RawKsuidMs,
}

#[napi]
impl JsKsuidMs {
  /// Create a new KsuidMs with current timestamp and optional 15-byte payload.
  #[napi(factory)]
  pub fn now(payload: Option<Uint8Array>) -> napi::Result<JsKsuidMs> {
    let p = parse_payload(payload, KSUID_MS_PAYLOAD_BYTES)?;
    Ok(JsKsuidMs {
      inner: RawKsuidMs::now(p.as_deref()),
    })
  }

  /// Create a KsuidMs with optional timestamp (in milliseconds since epoch) and optional 15-byte payload.
  #[napi(factory)]
  pub fn new(timestamp_ms: Option<f64>, payload: Option<Uint8Array>) -> napi::Result<JsKsuidMs> {
    let p = parse_payload(payload, KSUID_MS_PAYLOAD_BYTES)?;
    let m = timestamp_ms.map(|v| v as i64);
    Ok(JsKsuidMs {
      inner: RawKsuidMs::from_millis(m, p.as_deref()),
    })
  }

  /// Create a KsuidMs from milliseconds timestamp and optional payload.
  #[napi(factory)]
  pub fn from_milliseconds(ms: Option<f64>, payload: Option<Uint8Array>) -> napi::Result<JsKsuidMs> {
    let p = parse_payload(payload, KSUID_MS_PAYLOAD_BYTES)?;
    let m = ms.map(|v| v as i64);
    Ok(JsKsuidMs {
      inner: RawKsuidMs::from_millis(m, p.as_deref()),
    })
  }

  /// Parse a KsuidMs from a base62 string.
  #[napi(factory)]
  pub fn from_base62(base62: String) -> napi::Result<JsKsuidMs> {
    RawKsuidMs::from_base62(&base62)
      .map(|inner| JsKsuidMs { inner })
      .map_err(|e| napi::Error::from_reason(e.to_string()))
  }

  /// Parse a KsuidMs from string (implements FromStr).
  #[napi(factory)]
  pub fn from_str(base62: String) -> napi::Result<JsKsuidMs> {
    RawKsuidMs::from_str(&base62)
      .map(|inner| JsKsuidMs { inner })
      .map_err(|e| napi::Error::from_reason(e.to_string()))
  }

  /// Create a KsuidMs from 20 raw bytes.
  #[napi(factory)]
  pub fn from_bytes(bytes: Uint8Array) -> napi::Result<JsKsuidMs> {
    let slice = bytes.as_ref();
    if slice.len() != KSUID_BYTES {
      return Err(napi::Error::from_reason(format!(
        "KsuidMs bytes must be {} bytes",
        KSUID_BYTES
      )));
    }
    let mut arr = [0u8; KSUID_BYTES];
    arr.copy_from_slice(slice);
    Ok(JsKsuidMs {
      inner: RawKsuidMs::from_bytes(arr),
    })
  }

  /// Returns the base62 string representation.
  #[napi]
  pub fn to_base62(&self) -> String {
    self.inner.to_base62()
  }

  /// Returns string representation.
  #[napi]
  pub fn to_string(&self) -> String {
    self.inner.to_string()
  }

  /// Returns the 20 bytes of the KsuidMs.
  #[napi]
  pub fn bytes(&self) -> Uint8Array {
    Uint8Array::new(self.inner.bytes().to_vec())
  }

  /// Returns the 15 bytes of payload.
  #[napi]
  pub fn payload(&self) -> Uint8Array {
    Uint8Array::new(self.inner.payload().to_vec())
  }

  /// Returns the timestamp in milliseconds since UNIX epoch.
  #[napi]
  pub fn timestamp_milliseconds(&self) -> f64 {
    self.inner.timestamp_millis() as f64
  }

  /// Compare this KsuidMs with another KsuidMs (-1, 0, 1).
  #[napi]
  pub fn compare(&self, other: &JsKsuidMs) -> i32 {
    match self.inner.cmp(&other.inner) {
      std::cmp::Ordering::Less => -1,
      std::cmp::Ordering::Equal => 0,
      std::cmp::Ordering::Greater => 1,
    }
  }

  /// Check if equal to another KsuidMs.
  #[napi]
  pub fn equals(&self, other: &JsKsuidMs) -> bool {
    self.inner == other.inner
  }
}

/// Convenience function to generate a new Ksuid string.
#[napi]
pub fn generate_ksuid() -> String {
  RawKsuid::now(None).to_base62()
}

/// Convenience function to parse a base62 Ksuid string into a Ksuid object.
#[napi]
pub fn parse_ksuid(base62: String) -> napi::Result<JsKsuid> {
  JsKsuid::from_base62(base62)
}
