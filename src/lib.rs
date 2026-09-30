use napi::bindgen_prelude::*;
use napi_derive::napi;
use svix_ksuid::{Ksuid as InnerKsuid, KsuidLike};

#[napi]
pub struct Ksuid {
  inner: InnerKsuid,
}

#[napi]
impl Ksuid {
  /// Create a new Ksuid with an optional timestamp (in seconds since UNIX epoch) and optional 16-byte payload.
  #[napi(constructor)]
  pub fn new(timestamp: Option<i64>, payload: Option<Uint8Array>) -> Result<Self> {
    let payload_bytes = match payload {
      Some(arr) => {
        let slice: &[u8] = arr.as_ref();
        if slice.len() != InnerKsuid::PAYLOAD_BYTES {
          return Err(Error::new(
            Status::InvalidArg,
            format!(
              "Payload must be exactly {} bytes long, got {}",
              InnerKsuid::PAYLOAD_BYTES,
              slice.len()
            ),
          ));
        }
        let mut bytes = [0u8; InnerKsuid::PAYLOAD_BYTES];
        bytes.copy_from_slice(slice);
        Some(bytes)
      }
      None => None,
    };

    let inner = match (timestamp, payload_bytes) {
      (Some(ts), Some(p)) => InnerKsuid::from_seconds(Some(ts), Some(&p)),
      (Some(ts), None) => InnerKsuid::from_seconds(Some(ts), None),
      (None, Some(p)) => InnerKsuid::new_raw(
        std::time::SystemTime::now()
          .duration_since(std::time::UNIX_EPOCH)
          .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?
          .as_secs() as u32,
        Some(&p),
      ),
      (None, None) => InnerKsuid::from_seconds(None, None),
    };

    Ok(Ksuid { inner })
  }

  /// Timestamp is now, payload is randomly generated or provided.
  #[napi(factory)]
  pub fn now(payload: Option<Uint8Array>) -> Result<Self> {
    Self::new(None, payload)
  }

  /// Create a Ksuid from base62 string representation.
  #[napi(factory)]
  pub fn from_base62(base62: String) -> Result<Self> {
    let inner = InnerKsuid::from_base62(&base62).map_err(|e| {
      Error::new(
        Status::InvalidArg,
        format!("Invalid base62 KSUID string: {}", e),
      )
    })?;
    Ok(Ksuid { inner })
  }

  /// Alias for from_base62 / FromStr logic.
  #[napi(factory)]
  pub fn from_str(s: String) -> Result<Self> {
    Self::from_base62(s)
  }

  /// Create a Ksuid from 20 raw bytes.
  #[napi(factory)]
  pub fn from_bytes(bytes: Uint8Array) -> Result<Self> {
    let slice: &[u8] = bytes.as_ref();
    if slice.len() != 20 {
      return Err(Error::new(
        Status::InvalidArg,
        format!("KSUID bytes must be exactly 20 bytes long, got {}", slice.len()),
      ));
    }
    let mut arr = [0u8; 20];
    arr.copy_from_slice(slice);
    let inner = InnerKsuid::from_bytes(arr);
    Ok(Ksuid { inner })
  }

  /// Explicitly create Ksuid from timestamp seconds and optional payload.
  #[napi(factory)]
  pub fn from_seconds(seconds: Option<i64>, payload: Option<Uint8Array>) -> Result<Self> {
    Self::new(seconds, payload)
  }

  /// Base62 string representation.
  #[napi]
  pub fn to_string(&self) -> String {
    self.inner.to_string()
  }

  /// Base62 string representation (explicit alias).
  #[napi]
  pub fn to_base62(&self) -> String {
    self.inner.to_base62()
  }

  /// 20-byte slice representing the Ksuid.
  #[napi]
  pub fn bytes(&self) -> Uint8Array {
    Uint8Array::from(&self.inner.bytes()[..])
  }

  /// 16-byte payload portion of the Ksuid.
  #[napi]
  pub fn payload_bytes(&self) -> Uint8Array {
    Uint8Array::from(self.inner.payload())
  }

  /// Timestamp in seconds since UNIX epoch.
  #[napi]
  pub fn timestamp_seconds(&self) -> i64 {
    self.inner.timestamp_seconds()
  }

  /// Compare two Ksuids: returns -1 if self < other, 0 if equal, 1 if self > other.
  #[napi]
  pub fn compare_to(&self, other: &Ksuid) -> i32 {
    match self.inner.cmp(&other.inner) {
      std::cmp::Ordering::Less => -1,
      std::cmp::Ordering::Equal => 0,
      std::cmp::Ordering::Greater => 1,
    }
  }

  /// Check equality with another Ksuid instance.
  #[napi]
  pub fn equals(&self, other: &Ksuid) -> bool {
    self.inner == other.inner
  }
}
