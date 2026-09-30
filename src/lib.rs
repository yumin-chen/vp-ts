use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::str::FromStr;
use svix_ksuid::{Ksuid as RawKsuid, KsuidLike, KsuidMs as RawKsuidMs};

const KSUID_BYTES: usize = 20;
const KSUID_PAYLOAD_BYTES: usize = <RawKsuid as KsuidLike>::PAYLOAD_BYTES;
const KSUID_MS_PAYLOAD_BYTES: usize = <RawKsuidMs as KsuidLike>::PAYLOAD_BYTES;

const DEFAULT_CROCKFORD_ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

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

pub fn encode_base32_bytes(bytes: &[u8], alphabet: Option<String>) -> napi::Result<String> {
  let table = match alphabet {
    Some(ref s) => {
      let b = s.as_bytes();
      if b.len() != 32 {
        return Err(napi::Error::from_reason("Base32 alphabet must be exactly 32 characters"));
      }
      b.to_vec()
    }
    None => DEFAULT_CROCKFORD_ALPHABET.to_vec(),
  };

  let mut result = String::with_capacity((bytes.len() * 8 + 4) / 5);
  let mut bit_buf: u64 = 0;
  let mut bits_in_buf = 0;

  for &b in bytes {
    bit_buf = (bit_buf << 8) | (b as u64);
    bits_in_buf += 8;
    while bits_in_buf >= 5 {
      bits_in_buf -= 5;
      let val = ((bit_buf >> bits_in_buf) & 0x1F) as usize;
      result.push(table[val] as char);
    }
  }

  if bits_in_buf > 0 {
    let val = ((bit_buf << (5 - bits_in_buf)) & 0x1F) as usize;
    result.push(table[val] as char);
  }

  Ok(result)
}

pub fn decode_base32_bytes(encoded: &str, alphabet: Option<String>) -> napi::Result<Vec<u8>> {
  let decode_map: [i8; 256] = match alphabet {
    Some(ref s) => {
      let b = s.as_bytes();
      if b.len() != 32 {
        return Err(napi::Error::from_reason("Base32 alphabet must be exactly 32 characters"));
      }
      let mut map = [-1i8; 256];
      for (i, &ch) in b.iter().enumerate() {
        map[ch as usize] = i as i8;
      }
      map
    }
    None => {
      let mut map = [-1i8; 256];
      for i in 0..256 {
        let ch = i as u8 as char;
        let val: i8 = match ch {
          '0' | 'O' | 'o' => 0,
          '1' | 'I' | 'i' | 'L' | 'l' => 1,
          '2' => 2,
          '3' => 3,
          '4' => 4,
          '5' => 5,
          '6' => 6,
          '7' => 7,
          '8' => 8,
          '9' => 9,
          'A' | 'a' => 10,
          'B' | 'b' => 11,
          'C' | 'c' => 12,
          'D' | 'd' => 13,
          'E' | 'e' => 14,
          'F' | 'f' => 15,
          'G' | 'g' => 16,
          'H' | 'h' => 17,
          'J' | 'j' => 18,
          'K' | 'k' => 19,
          'M' | 'm' => 20,
          'N' | 'n' => 21,
          'P' | 'p' => 22,
          'Q' | 'q' => 23,
          'R' | 'r' => 24,
          'S' | 's' => 25,
          'T' | 't' => 26,
          'V' | 'v' => 27,
          'W' | 'w' => 28,
          'X' | 'x' => 29,
          'Y' | 'y' => 30,
          'Z' | 'z' => 31,
          _ => -1,
        };
        map[i] = val;
      }
      map
    }
  };

  let mut bit_buf: u64 = 0;
  let mut bits_in_buf = 0;
  let mut out = Vec::new();

  for (idx, &byte) in encoded.as_bytes().iter().enumerate() {
    let val = decode_map[byte as usize];
    if val < 0 {
      return Err(napi::Error::from_reason(format!(
        "Invalid Base32 character '{}' at index {}",
        byte as char, idx
      )));
    }
    bit_buf = (bit_buf << 5) | (val as u64);
    bits_in_buf += 5;
    if bits_in_buf >= 8 {
      bits_in_buf -= 8;
      out.push((bit_buf >> bits_in_buf) as u8);
    }
  }

  Ok(out)
}

pub fn encode_u64_crockford(mut n: u64, alphabet: Option<String>) -> napi::Result<String> {
  let table = match alphabet {
    Some(ref s) => {
      let b = s.as_bytes();
      if b.len() != 32 {
        return Err(napi::Error::from_reason("Base32 alphabet must be exactly 32 characters"));
      }
      b.to_vec()
    }
    None => DEFAULT_CROCKFORD_ALPHABET.to_vec(),
  };

  if n == 0 {
    return Ok((table[0] as char).to_string());
  }

  let mut chars = Vec::new();
  while n > 0 {
    let rem = (n % 32) as usize;
    chars.push(table[rem] as char);
    n /= 32;
  }
  chars.reverse();
  Ok(chars.into_iter().collect())
}

pub fn decode_u64_crockford(input: &str, alphabet: Option<String>) -> napi::Result<u64> {
  if input.is_empty() {
    return Err(napi::Error::from_reason("Empty string"));
  }
  let decode_map: [i8; 256] = match alphabet {
    Some(ref s) => {
      let b = s.as_bytes();
      if b.len() != 32 {
        return Err(napi::Error::from_reason("Base32 alphabet must be exactly 32 characters"));
      }
      let mut map = [-1i8; 256];
      for (i, &ch) in b.iter().enumerate() {
        map[ch as usize] = i as i8;
      }
      map
    }
    None => {
      let mut map = [-1i8; 256];
      for i in 0..256 {
        let ch = i as u8 as char;
        let val: i8 = match ch {
          '0' | 'O' | 'o' => 0,
          '1' | 'I' | 'i' | 'L' | 'l' => 1,
          '2' => 2,
          '3' => 3,
          '4' => 4,
          '5' => 5,
          '6' => 6,
          '7' => 7,
          '8' => 8,
          '9' => 9,
          'A' | 'a' => 10,
          'B' | 'b' => 11,
          'C' | 'c' => 12,
          'D' | 'd' => 13,
          'E' | 'e' => 14,
          'F' | 'f' => 15,
          'G' | 'g' => 16,
          'H' | 'h' => 17,
          'J' | 'j' => 18,
          'K' | 'k' => 19,
          'M' | 'm' => 20,
          'N' | 'n' => 21,
          'P' | 'p' => 22,
          'Q' | 'q' => 23,
          'R' | 'r' => 24,
          'S' | 's' => 25,
          'T' | 't' => 26,
          'V' | 'v' => 27,
          'W' | 'w' => 28,
          'X' | 'x' => 29,
          'Y' | 'y' => 30,
          'Z' | 'z' => 31,
          _ => -1,
        };
        map[i] = val;
      }
      map
    }
  };

  let mut n: u64 = 0;
  for (idx, &byte) in input.as_bytes().iter().enumerate() {
    let digit = decode_map[byte as usize];
    if digit < 0 {
      return Err(napi::Error::from_reason(format!(
        "Invalid Base32 character '{}' at index {}",
        byte as char, idx
      )));
    }
    n = n
      .checked_mul(32)
      .and_then(|val| val.checked_add(digit as u64))
      .ok_or_else(|| napi::Error::from_reason("Integer overflow decoding Crockford Base32"))?;
  }

  Ok(n)
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

  /// Parse a Ksuid from a base62 string (optional custom alphabet).
  #[napi(factory)]
  pub fn from_base62(base62: String, alphabet: Option<String>) -> napi::Result<JsKsuid> {
    match alphabet {
      None => RawKsuid::from_base62(&base62)
        .map(|inner| JsKsuid { inner })
        .map_err(|e| napi::Error::from_reason(e.to_string())),
      Some(ref alph) => {
        if alph.as_bytes().len() != 62 {
          return Err(napi::Error::from_reason("Base62 alphabet must be exactly 62 characters"));
        }
        let bytes = base_encode::from_str(&base62, 62, alph.as_bytes())
          .ok_or_else(|| napi::Error::from_reason("Failed to decode Base62 string"))?;
        if bytes.len() != KSUID_BYTES {
          return Err(napi::Error::from_reason("Decoded bytes length mismatch for Ksuid"));
        }
        let mut arr = [0u8; KSUID_BYTES];
        arr.copy_from_slice(&bytes);
        Ok(JsKsuid {
          inner: RawKsuid::from_bytes(arr),
        })
      }
    }
  }

  /// Parse a Ksuid from string (implements FromStr).
  #[napi(factory)]
  pub fn from_str(base62: String) -> napi::Result<JsKsuid> {
    RawKsuid::from_str(&base62)
      .map(|inner| JsKsuid { inner })
      .map_err(|e| napi::Error::from_reason(e.to_string()))
  }

  /// Parse a Ksuid from a Crockford Base32 string (optional custom alphabet).
  #[napi(factory)]
  pub fn from_base32(encoded: String, alphabet: Option<String>) -> napi::Result<JsKsuid> {
    let bytes = decode_base32_bytes(&encoded, alphabet)?;
    if bytes.len() != KSUID_BYTES {
      return Err(napi::Error::from_reason("Decoded Base32 length mismatch for Ksuid"));
    }
    let mut arr = [0u8; KSUID_BYTES];
    arr.copy_from_slice(&bytes);
    Ok(JsKsuid {
      inner: RawKsuid::from_bytes(arr),
    })
  }

  /// Alias for from_base32.
  #[napi(factory)]
  pub fn from_crockford_base32(encoded: String, alphabet: Option<String>) -> napi::Result<JsKsuid> {
    Self::from_base32(encoded, alphabet)
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

  /// Returns the base62 string representation (optional custom alphabet).
  #[napi]
  pub fn to_base62(&self, alphabet: Option<String>) -> napi::Result<String> {
    match alphabet {
      None => Ok(self.inner.to_base62()),
      Some(ref alph) => {
        if alph.as_bytes().len() != 62 {
          return Err(napi::Error::from_reason("Base62 alphabet must be exactly 62 characters"));
        }
        base_encode::to_string(self.inner.bytes(), 62, alph.as_bytes())
          .ok_or_else(|| napi::Error::from_reason("Failed to encode Base62 string"))
      }
    }
  }

  /// Returns Crockford Base32 string representation (optional custom alphabet).
  #[napi]
  pub fn to_base32(&self, alphabet: Option<String>) -> napi::Result<String> {
    encode_base32_bytes(self.inner.bytes(), alphabet)
  }

  /// Alias for to_base32.
  #[napi]
  pub fn to_crockford_base32(&self, alphabet: Option<String>) -> napi::Result<String> {
    self.to_base32(alphabet)
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

  /// Parse a KsuidMs from a base62 string (optional custom alphabet).
  #[napi(factory)]
  pub fn from_base62(base62: String, alphabet: Option<String>) -> napi::Result<JsKsuidMs> {
    match alphabet {
      None => RawKsuidMs::from_base62(&base62)
        .map(|inner| JsKsuidMs { inner })
        .map_err(|e| napi::Error::from_reason(e.to_string())),
      Some(ref alph) => {
        if alph.as_bytes().len() != 62 {
          return Err(napi::Error::from_reason("Base62 alphabet must be exactly 62 characters"));
        }
        let bytes = base_encode::from_str(&base62, 62, alph.as_bytes())
          .ok_or_else(|| napi::Error::from_reason("Failed to decode Base62 string"))?;
        if bytes.len() != KSUID_BYTES {
          return Err(napi::Error::from_reason("Decoded bytes length mismatch for KsuidMs"));
        }
        let mut arr = [0u8; KSUID_BYTES];
        arr.copy_from_slice(&bytes);
        Ok(JsKsuidMs {
          inner: RawKsuidMs::from_bytes(arr),
        })
      }
    }
  }

  /// Parse a KsuidMs from string (implements FromStr).
  #[napi(factory)]
  pub fn from_str(base62: String) -> napi::Result<JsKsuidMs> {
    RawKsuidMs::from_str(&base62)
      .map(|inner| JsKsuidMs { inner })
      .map_err(|e| napi::Error::from_reason(e.to_string()))
  }

  /// Parse a KsuidMs from a Crockford Base32 string (optional custom alphabet).
  #[napi(factory)]
  pub fn from_base32(encoded: String, alphabet: Option<String>) -> napi::Result<JsKsuidMs> {
    let bytes = decode_base32_bytes(&encoded, alphabet)?;
    if bytes.len() != KSUID_BYTES {
      return Err(napi::Error::from_reason("Decoded Base32 length mismatch for KsuidMs"));
    }
    let mut arr = [0u8; KSUID_BYTES];
    arr.copy_from_slice(&bytes);
    Ok(JsKsuidMs {
      inner: RawKsuidMs::from_bytes(arr),
    })
  }

  /// Alias for from_base32.
  #[napi(factory)]
  pub fn from_crockford_base32(encoded: String, alphabet: Option<String>) -> napi::Result<JsKsuidMs> {
    Self::from_base32(encoded, alphabet)
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

  /// Returns the base62 string representation (optional custom alphabet).
  #[napi]
  pub fn to_base62(&self, alphabet: Option<String>) -> napi::Result<String> {
    match alphabet {
      None => Ok(self.inner.to_base62()),
      Some(ref alph) => {
        if alph.as_bytes().len() != 62 {
          return Err(napi::Error::from_reason("Base62 alphabet must be exactly 62 characters"));
        }
        base_encode::to_string(self.inner.bytes(), 62, alph.as_bytes())
          .ok_or_else(|| napi::Error::from_reason("Failed to encode Base62 string"))
      }
    }
  }

  /// Returns Crockford Base32 string representation (optional custom alphabet).
  #[napi]
  pub fn to_base32(&self, alphabet: Option<String>) -> napi::Result<String> {
    encode_base32_bytes(self.inner.bytes(), alphabet)
  }

  /// Alias for to_base32.
  #[napi]
  pub fn to_crockford_base32(&self, alphabet: Option<String>) -> napi::Result<String> {
    self.to_base32(alphabet)
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
pub fn generate_ksuid(alphabet: Option<String>) -> napi::Result<String> {
  let ksuid = RawKsuid::now(None);
  match alphabet {
    None => Ok(ksuid.to_base62()),
    Some(ref alph) => {
      if alph.as_bytes().len() == 32 {
        encode_base32_bytes(ksuid.bytes(), alphabet)
      } else if alph.as_bytes().len() == 62 {
        base_encode::to_string(ksuid.bytes(), 62, alph.as_bytes())
          .ok_or_else(|| napi::Error::from_reason("Failed to encode Base62 string"))
      } else {
        Err(napi::Error::from_reason("Alphabet must be 32 or 62 characters"))
      }
    }
  }
}

/// Convenience function to parse a string into a Ksuid object.
#[napi]
pub fn parse_ksuid(encoded: String, alphabet: Option<String>) -> napi::Result<JsKsuid> {
  match alphabet {
    None => JsKsuid::from_base62(encoded, None),
    Some(ref alph) => {
      if alph.as_bytes().len() == 32 {
        JsKsuid::from_base32(encoded, alphabet)
      } else if alph.as_bytes().len() == 62 {
        JsKsuid::from_base62(encoded, alphabet)
      } else {
        Err(napi::Error::from_reason("Alphabet must be 32 or 62 characters"))
      }
    }
  }
}

/// Encode a u64 number using Crockford Base32.
#[napi]
pub fn encode_crockford_base32(n: f64, alphabet: Option<String>) -> napi::Result<String> {
  encode_u64_crockford(n as u64, alphabet)
}

/// Decode a Crockford Base32 string to number.
#[napi]
pub fn decode_crockford_base32(encoded: String, alphabet: Option<String>) -> napi::Result<f64> {
  let val = decode_u64_crockford(&encoded, alphabet)?;
  Ok(val as f64)
}

/// Encode raw bytes into Crockford Base32 (optional custom alphabet).
#[napi]
pub fn encode_base32_bytes_js(bytes: Uint8Array, alphabet: Option<String>) -> napi::Result<String> {
  encode_base32_bytes(bytes.as_ref(), alphabet)
}

/// Decode a Crockford Base32 string into raw bytes (optional custom alphabet).
#[napi]
pub fn decode_base32_bytes_js(encoded: String, alphabet: Option<String>) -> napi::Result<Uint8Array> {
  let bytes = decode_base32_bytes(&encoded, alphabet)?;
  Ok(Uint8Array::new(bytes))
}

/// Shuffles an alphabet string deterministically with a seed.
#[napi]
pub fn shuffle_alphabet(alphabet: String, seed: Option<f64>) -> napi::Result<String> {
  let mut chars: Vec<char> = alphabet.chars().collect();
  let len = chars.len();
  if len == 0 {
    return Err(napi::Error::from_reason("Alphabet cannot be empty"));
  }

  let mut s = seed.unwrap_or(42.0) as u64;
  for i in (1..len).rev() {
    s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    let j = (s as usize) % (i + 1);
    chars.swap(i, j);
  }

  Ok(chars.into_iter().collect())
}
