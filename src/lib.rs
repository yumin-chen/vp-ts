use jiff::Timestamp;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::str::FromStr;
use svix_ksuid::{Ksuid as RawKsuid, KsuidLike, KsuidMs as RawKsuidMs};

#[napi]
pub struct Ksuid {
  inner: RawKsuid,
}

#[napi]
impl Ksuid {
  /// Create a new Ksuid.
  /// If no arguments are passed, generates a new KSUID with current timestamp and random payload.
  /// If a string is passed, parses it as a Base62 KSUID string.
  /// If a Uint8Array or Buffer is passed, creates KSUID from 20 raw bytes.
  #[napi(constructor)]
  pub fn constructor(value: Option<Either<String, Uint8Array>>) -> Result<Self> {
    match value {
      None => Ok(Self {
        inner: RawKsuid::now(None),
      }),
      Some(Either::A(s)) => {
        let inner = RawKsuid::from_str(&s)
          .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid Base62 KSUID string: {}", e)))?;
        Ok(Self { inner })
      }
      Some(Either::B(bytes)) => {
        let slice: &[u8] = bytes.as_ref();
        if slice.len() != 20 {
          return Err(Error::new(
            Status::InvalidArg,
            format!("Expected 20 bytes for KSUID, got {}", slice.len()),
          ));
        }
        let mut arr = [0u8; 20];
        arr.copy_from_slice(slice);
        Ok(Self {
          inner: RawKsuid::from_bytes(arr),
        })
      }
    }
  }

  /// Generates a new KSUID with current timestamp and optional payload.
  #[napi(factory)]
  pub fn now(payload: Option<Uint8Array>) -> Self {
    let payload_ref = payload.as_ref().map(|p| p.as_ref());
    Self {
      inner: RawKsuid::now(payload_ref),
    }
  }

  /// Create a KSUID from optional timestamp (seconds or ISO string) and optional payload.
  #[napi(factory)]
  pub fn new(timestamp: Option<Either<f64, String>>, payload: Option<Uint8Array>) -> Result<Self> {
    let payload_ref = payload.as_ref().map(|p| p.as_ref());
    let ts = match timestamp {
      None => None,
      Some(Either::A(num)) => {
        // If > 1e11, assume timestamp in milliseconds (e.g. Date.now())
        let secs_i64 = if num > 1e11 {
          (num / 1000.0) as i64
        } else {
          num as i64
        };
        Some(
          Timestamp::from_second(secs_i64)
            .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid timestamp: {}", e)))?,
        )
      }
      Some(Either::B(iso_str)) => {
        let ts = Timestamp::from_str(&iso_str)
          .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid ISO timestamp string: {}", e)))?;
        Some(ts)
      }
    };

    Ok(Self {
      inner: RawKsuid::new(ts, payload_ref),
    })
  }

  /// Parses a KSUID from a Base62 string.
  #[napi(factory)]
  pub fn from_base62(base62: String) -> Result<Self> {
    let inner = RawKsuid::from_str(&base62)
      .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid Base62 KSUID string: {}", e)))?;
    Ok(Self { inner })
  }

  /// Parses a KSUID from a Base62 string (alias for `fromBase62`).
  #[napi(factory)]
  pub fn from_string(s: String) -> Result<Self> {
    Self::from_base62(s)
  }

  /// Create a KSUID from 20 raw bytes.
  #[napi(factory)]
  pub fn from_bytes(bytes: Uint8Array) -> Result<Self> {
    let slice: &[u8] = bytes.as_ref();
    if slice.len() != 20 {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Expected 20 bytes for KSUID, got {}", slice.len()),
      ));
    }
    let mut arr = [0u8; 20];
    arr.copy_from_slice(slice);
    Ok(Self {
      inner: RawKsuid::from_bytes(arr),
    })
  }

  /// Create a KSUID from seconds timestamp and optional payload.
  #[napi(factory)]
  pub fn from_seconds(seconds: Option<f64>, payload: Option<Uint8Array>) -> Self {
    let payload_ref = payload.as_ref().map(|p| p.as_ref());
    let secs_i64 = seconds.map(|s| s as i64);
    Self {
      inner: RawKsuid::from_seconds(secs_i64, payload_ref),
    }
  }

  /// Convert KSUID to Base62 string.
  #[napi]
  pub fn to_base62(&self) -> String {
    self.inner.to_base62()
  }

  /// Convert KSUID to string (Base62 representation).
  #[napi]
  pub fn to_string(&self) -> String {
    self.inner.to_string()
  }

  /// Returns the raw 20 bytes of the KSUID as a Uint8Array.
  #[napi]
  pub fn bytes(&self) -> Uint8Array {
    Uint8Array::from(&self.inner.bytes()[..])
  }

  /// Returns the 16-byte payload of the KSUID as a Uint8Array.
  #[napi]
  pub fn payload(&self) -> Uint8Array {
    Uint8Array::from(&self.inner.payload()[..])
  }

  /// Returns the timestamp in seconds since Unix epoch.
  #[napi]
  pub fn timestamp_seconds(&self) -> i64 {
    self.inner.timestamp_seconds()
  }

  /// Returns the timestamp as an ISO string.
  #[napi]
  pub fn timestamp_iso(&self) -> String {
    let ts = self.inner.timestamp::<Timestamp>();
    ts.to_string()
  }

  /// Compare two KSUIDs. Returns -1 if self < other, 0 if self == other, 1 if self > other.
  #[napi]
  pub fn compare(&self, other: &Ksuid) -> i32 {
    match self.inner.cmp(&other.inner) {
      std::cmp::Ordering::Less => -1,
      std::cmp::Ordering::Equal => 0,
      std::cmp::Ordering::Greater => 1,
    }
  }

  /// Check equality with another KSUID.
  #[napi]
  pub fn equals(&self, other: &Ksuid) -> bool {
    self.inner == other.inner
  }
}

#[napi]
pub struct KsuidMs {
  inner: RawKsuidMs,
}

#[napi]
impl KsuidMs {
  /// Create a new KsuidMs.
  #[napi(constructor)]
  pub fn constructor(value: Option<Either<String, Uint8Array>>) -> Result<Self> {
    match value {
      None => Ok(Self {
        inner: RawKsuidMs::now(None),
      }),
      Some(Either::A(s)) => {
        let inner = RawKsuidMs::from_str(&s)
          .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid Base62 KsuidMs string: {}", e)))?;
        Ok(Self { inner })
      }
      Some(Either::B(bytes)) => {
        let slice: &[u8] = bytes.as_ref();
        if slice.len() != 20 {
          return Err(Error::new(
            Status::InvalidArg,
            format!("Expected 20 bytes for KsuidMs, got {}", slice.len()),
          ));
        }
        let mut arr = [0u8; 20];
        arr.copy_from_slice(slice);
        Ok(Self {
          inner: RawKsuidMs::from_bytes(arr),
        })
      }
    }
  }

  #[napi(factory)]
  pub fn now(payload: Option<Uint8Array>) -> Self {
    let payload_ref = payload.as_ref().map(|p| p.as_ref());
    Self {
      inner: RawKsuidMs::now(payload_ref),
    }
  }

  #[napi(factory)]
  pub fn new(timestamp: Option<Either<f64, String>>, payload: Option<Uint8Array>) -> Result<Self> {
    let payload_ref = payload.as_ref().map(|p| p.as_ref());
    let ts = match timestamp {
      None => None,
      Some(Either::A(num)) => {
        // If < 1e11, assume timestamp in seconds and convert to ms
        let ms_i64 = if num < 1e11 {
          (num * 1000.0) as i64
        } else {
          num as i64
        };
        Some(
          Timestamp::from_millisecond(ms_i64)
            .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid timestamp milliseconds: {}", e)))?,
        )
      }
      Some(Either::B(iso_str)) => {
        let ts = Timestamp::from_str(&iso_str)
          .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid ISO timestamp string: {}", e)))?;
        Some(ts)
      }
    };

    Ok(Self {
      inner: RawKsuidMs::new(ts, payload_ref),
    })
  }

  #[napi(factory)]
  pub fn from_base62(base62: String) -> Result<Self> {
    let inner = RawKsuidMs::from_str(&base62)
      .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid Base62 KsuidMs string: {}", e)))?;
    Ok(Self { inner })
  }

  #[napi(factory)]
  pub fn from_string(s: String) -> Result<Self> {
    Self::from_base62(s)
  }

  #[napi(factory)]
  pub fn from_bytes(bytes: Uint8Array) -> Result<Self> {
    let slice: &[u8] = bytes.as_ref();
    if slice.len() != 20 {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Expected 20 bytes for KsuidMs, got {}", slice.len()),
      ));
    }
    let mut arr = [0u8; 20];
    arr.copy_from_slice(slice);
    Ok(Self {
      inner: RawKsuidMs::from_bytes(arr),
    })
  }

  #[napi(factory)]
  pub fn from_seconds(seconds: Option<f64>, payload: Option<Uint8Array>) -> Self {
    let payload_ref = payload.as_ref().map(|p| p.as_ref());
    let secs_i64 = seconds.map(|s| s as i64);
    Self {
      inner: RawKsuidMs::from_seconds(secs_i64, payload_ref),
    }
  }

  #[napi(factory)]
  pub fn from_milliseconds(millis: Option<f64>, payload: Option<Uint8Array>) -> Self {
    let payload_ref = payload.as_ref().map(|p| p.as_ref());
    let ms_i64 = millis.map(|m| m as i64);
    Self {
      inner: RawKsuidMs::from_millis(ms_i64, payload_ref),
    }
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
  pub fn bytes(&self) -> Uint8Array {
    Uint8Array::from(&self.inner.bytes()[..])
  }

  #[napi]
  pub fn payload(&self) -> Uint8Array {
    Uint8Array::from(&self.inner.payload()[..])
  }

  #[napi]
  pub fn timestamp_seconds(&self) -> i64 {
    self.inner.timestamp_seconds()
  }

  #[napi]
  pub fn timestamp_milliseconds(&self) -> i64 {
    let ts = self.inner.timestamp::<Timestamp>();
    ts.as_millisecond()
  }

  #[napi]
  pub fn timestamp_iso(&self) -> String {
    let ts = self.inner.timestamp::<Timestamp>();
    ts.to_string()
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
