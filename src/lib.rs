use jiff::Timestamp;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::str::FromStr;
use svix_ksuid::{Ksuid as RawKsuid, KsuidLike, KsuidMs as RawKsuidMs};

const DEFAULT_CROCKFORD_ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

fn shuffle_alphabet(alphabet: &[u8; 32], seed: &str) -> [u8; 32] {
  let mut shuffled = *alphabet;
  let mut state: u64 = 0xcbf29ce484222325;
  for b in seed.bytes() {
    state ^= b as u64;
    state = state.wrapping_mul(0x100000001b3);
  }
  if state == 0 {
    state = 1;
  }

  for i in (1..32).rev() {
    state ^= state >> 12;
    state ^= state << 25;
    state ^= state >> 27;
    let rnd = state.wrapping_mul(0x2545F4914F6CDD1D);
    let j = (rnd as usize) % (i + 1);
    shuffled.swap(i, j);
  }
  shuffled
}

#[napi]
pub struct CrockfordBase32 {
  alphabet: [u8; 32],
  decode_map: [i8; 256],
}

impl CrockfordBase32 {
  fn from_alphabet_bytes(alpha: &[u8; 32], is_default: bool) -> Self {
    let mut decode_map = [-1i8; 256];
    for (idx, &byte) in alpha.iter().enumerate() {
      decode_map[byte as usize] = idx as i8;
      if (byte as char).is_ascii_uppercase() {
        let lower = (byte as char).to_ascii_lowercase() as u8;
        decode_map[lower as usize] = idx as i8;
      } else if (byte as char).is_ascii_lowercase() {
        let upper = (byte as char).to_ascii_uppercase() as u8;
        decode_map[upper as usize] = idx as i8;
      }
    }

    if is_default {
      decode_map[b'o' as usize] = 0;
      decode_map[b'O' as usize] = 0;
      decode_map[b'i' as usize] = 1;
      decode_map[b'I' as usize] = 1;
      decode_map[b'l' as usize] = 1;
      decode_map[b'L' as usize] = 1;
    }

    Self {
      alphabet: *alpha,
      decode_map,
    }
  }

  fn decode_char(&self, ch: u8) -> Result<u8> {
    let val = self.decode_map[ch as usize];
    if val < 0 {
      Err(Error::new(
        Status::InvalidArg,
        format!("Invalid Crockford Base32 character: '{}'", ch as char),
      ))
    } else {
      Ok(val as u8)
    }
  }

  fn encode_bytes(&self, bytes: &[u8]) -> String {
    let mut result = String::with_capacity((bytes.len() * 8 + 4) / 5);
    let mut bit_buf = 0u64;
    let mut bit_cnt = 0;
    for &byte in bytes {
      bit_buf = (bit_buf << 8) | (byte as u64);
      bit_cnt += 8;
      while bit_cnt >= 5 {
        bit_cnt -= 5;
        let index = ((bit_buf >> bit_cnt) & 0x1F) as usize;
        result.push(self.alphabet[index] as char);
      }
    }
    if bit_cnt > 0 {
      let index = ((bit_buf << (5 - bit_cnt)) & 0x1F) as usize;
      result.push(self.alphabet[index] as char);
    }
    result
  }

  fn decode_bytes(&self, s: &str) -> Result<Vec<u8>> {
    let mut bit_buf = 0u64;
    let mut bit_cnt = 0;
    let mut out = Vec::new();
    for ch in s.bytes() {
      let val = self.decode_char(ch)?;
      bit_buf = (bit_buf << 5) | (val as u64);
      bit_cnt += 5;
      if bit_cnt >= 8 {
        bit_cnt -= 8;
        out.push(((bit_buf >> bit_cnt) & 0xFF) as u8);
      }
    }
    Ok(out)
  }

  fn encode_u64_internal(&self, mut n: u64) -> String {
    if n == 0 {
      return (self.alphabet[0] as char).to_string();
    }
    let mut digits = Vec::new();
    while n > 0 {
      let idx = (n % 32) as usize;
      digits.push(self.alphabet[idx] as char);
      n /= 32;
    }
    digits.into_iter().rev().collect()
  }

  fn decode_u64_internal(&self, s: &str) -> Result<u64> {
    if s.is_empty() {
      return Err(Error::new(Status::InvalidArg, "Empty Base32 string"));
    }
    let mut n: u64 = 0;
    for ch in s.bytes() {
      let digit = self.decode_char(ch)?;
      n = n
        .checked_mul(32)
        .and_then(|m| m.checked_add(digit as u64))
        .ok_or_else(|| Error::new(Status::InvalidArg, "Base32 string out of range for u64"))?;
    }
    Ok(n)
  }
}

#[napi]
impl CrockfordBase32 {
  /// Create a Crockford Base32 encoder with default or custom 32-character alphabet.
  #[napi(constructor)]
  pub fn constructor(custom_alphabet: Option<String>) -> Result<Self> {
    if let Some(alpha) = custom_alphabet {
      Self::with_alphabet(alpha)
    } else {
      Ok(Self::default_encoder())
    }
  }

  #[napi(factory)]
  pub fn default_encoder() -> Self {
    Self::from_alphabet_bytes(DEFAULT_CROCKFORD_ALPHABET, true)
  }

  #[napi(factory)]
  pub fn with_alphabet(alphabet_str: String) -> Result<Self> {
    let bytes = alphabet_str.as_bytes();
    if bytes.len() != 32 {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Alphabet must be exactly 32 characters, got {}", bytes.len()),
      ));
    }
    let mut arr = [0u8; 32];
    arr.copy_from_slice(bytes);
    Ok(Self::from_alphabet_bytes(&arr, false))
  }

  /// Create a new CrockfordBase32 encoder by shuffling the current alphabet with a seed string.
  #[napi]
  pub fn shuffle(&self, seed: String) -> Self {
    let shuffled = shuffle_alphabet(&self.alphabet, &seed);
    Self::from_alphabet_bytes(&shuffled, false)
  }

  /// Returns the current 32-character alphabet string.
  #[napi]
  pub fn get_alphabet(&self) -> String {
    String::from_utf8_lossy(&self.alphabet).to_string()
  }

  /// Encode a u64 number or a Uint8Array byte buffer into Crockford Base32.
  #[napi]
  pub fn encode(&self, input: Either<f64, Uint8Array>) -> String {
    match input {
      Either::A(num) => self.encode_u64_internal(num as u64),
      Either::B(bytes) => self.encode_bytes(bytes.as_ref()),
    }
  }

  /// Decode a Crockford Base32 string into a Uint8Array buffer.
  #[napi]
  pub fn decode(&self, input: String) -> Result<Uint8Array> {
    let bytes = self.decode_bytes(&input)?;
    Ok(Uint8Array::from(bytes.as_slice()))
  }

  /// Decode a Crockford Base32 string into a u64 number.
  #[napi]
  pub fn decode_u64(&self, input: String) -> Result<f64> {
    let val = self.decode_u64_internal(&input)?;
    Ok(val as f64)
  }
}

#[napi]
pub struct Ksuid {
  inner: RawKsuid,
}

#[napi]
impl Ksuid {
  /// Create a new Ksuid.
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

  #[napi(factory)]
  pub fn now(payload: Option<Uint8Array>) -> Self {
    let payload_ref = payload.as_ref().map(|p| p.as_ref());
    Self {
      inner: RawKsuid::now(payload_ref),
    }
  }

  #[napi(factory)]
  pub fn new(timestamp: Option<Either<f64, String>>, payload: Option<Uint8Array>) -> Result<Self> {
    let payload_ref = payload.as_ref().map(|p| p.as_ref());
    let ts = match timestamp {
      None => None,
      Some(Either::A(num)) => {
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

  #[napi(factory)]
  pub fn from_base62(base62: String) -> Result<Self> {
    let inner = RawKsuid::from_str(&base62)
      .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid Base62 KSUID string: {}", e)))?;
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
        format!("Expected 20 bytes for KSUID, got {}", slice.len()),
      ));
    }
    let mut arr = [0u8; 20];
    arr.copy_from_slice(slice);
    Ok(Self {
      inner: RawKsuid::from_bytes(arr),
    })
  }

  #[napi(factory)]
  pub fn from_seconds(seconds: Option<f64>, payload: Option<Uint8Array>) -> Self {
    let payload_ref = payload.as_ref().map(|p| p.as_ref());
    let secs_i64 = seconds.map(|s| s as i64);
    Self {
      inner: RawKsuid::from_seconds(secs_i64, payload_ref),
    }
  }

  #[napi(factory)]
  pub fn from_crockford_base32(s: String, encoder: Option<&CrockfordBase32>) -> Result<Self> {
    let default_enc;
    let enc = match encoder {
      Some(e) => e,
      None => {
        default_enc = CrockfordBase32::default_encoder();
        &default_enc
      }
    };
    let bytes = enc.decode_bytes(&s)?;
    if bytes.len() != 20 {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Expected 20 bytes for KSUID from Crockford Base32, got {}", bytes.len()),
      ));
    }
    let mut arr = [0u8; 20];
    arr.copy_from_slice(&bytes);
    Ok(Self {
      inner: RawKsuid::from_bytes(arr),
    })
  }

  #[napi]
  pub fn to_base62(&self) -> String {
    self.inner.to_base62()
  }

  #[napi]
  pub fn to_crockford_base32(&self, encoder: Option<&CrockfordBase32>) -> String {
    let default_enc;
    let enc = match encoder {
      Some(e) => e,
      None => {
        default_enc = CrockfordBase32::default_encoder();
        &default_enc
      }
    };
    enc.encode_bytes(self.inner.bytes().as_slice())
  }

  /// Convert Ksuid to string representation.
  /// Accepts optional encoding ("base62", "crockfordBase32", "base32") and optional custom encoder.
  #[napi]
  pub fn to_string(&self, encoding: Option<String>, encoder: Option<&CrockfordBase32>) -> Result<String> {
    match encoding.as_deref() {
      Some("crockfordBase32") | Some("crockford_base32") | Some("base32") => {
        Ok(self.to_crockford_base32(encoder))
      }
      Some("base62") | None => Ok(self.inner.to_base62()),
      Some(other) => Err(Error::new(
        Status::InvalidArg,
        format!("Unsupported encoding: '{}'. Expected 'base62' or 'crockfordBase32'", other),
      )),
    }
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
  pub fn timestamp_iso(&self) -> String {
    let ts = self.inner.timestamp::<Timestamp>();
    ts.to_string()
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
  inner: RawKsuidMs,
}

#[napi]
impl KsuidMs {
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

  #[napi(factory)]
  pub fn from_crockford_base32(s: String, encoder: Option<&CrockfordBase32>) -> Result<Self> {
    let default_enc;
    let enc = match encoder {
      Some(e) => e,
      None => {
        default_enc = CrockfordBase32::default_encoder();
        &default_enc
      }
    };
    let bytes = enc.decode_bytes(&s)?;
    if bytes.len() != 20 {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Expected 20 bytes for KsuidMs from Crockford Base32, got {}", bytes.len()),
      ));
    }
    let mut arr = [0u8; 20];
    arr.copy_from_slice(&bytes);
    Ok(Self {
      inner: RawKsuidMs::from_bytes(arr),
    })
  }

  #[napi]
  pub fn to_base62(&self) -> String {
    self.inner.to_base62()
  }

  #[napi]
  pub fn to_crockford_base32(&self, encoder: Option<&CrockfordBase32>) -> String {
    let default_enc;
    let enc = match encoder {
      Some(e) => e,
      None => {
        default_enc = CrockfordBase32::default_encoder();
        &default_enc
      }
    };
    enc.encode_bytes(self.inner.bytes().as_slice())
  }

  /// Convert KsuidMs to string representation.
  /// Accepts optional encoding ("base62", "crockfordBase32", "base32") and optional custom encoder.
  #[napi]
  pub fn to_string(&self, encoding: Option<String>, encoder: Option<&CrockfordBase32>) -> Result<String> {
    match encoding.as_deref() {
      Some("crockfordBase32") | Some("crockford_base32") | Some("base32") => {
        Ok(self.to_crockford_base32(encoder))
      }
      Some("base62") | None => Ok(self.inner.to_base62()),
      Some(other) => Err(Error::new(
        Status::InvalidArg,
        format!("Unsupported encoding: '{}'. Expected 'base62' or 'crockfordBase32'", other),
      )),
    }
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
