use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Mutex;
use svix_ksuid::{Ksuid as InnerKsuid, KsuidLike, KsuidMs as InnerKsuidMs};

const DEFAULT_CROCKFORD_ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";
const DEFAULT_BASE36_ALPHABET: &[u8; 36] = b"0123456789abcdefghijklmnopqrstuvwxyz";

static DEFAULT_ENCODING: AtomicU8 = AtomicU8::new(0); // 0 = Base62, 1 = Crockford, 2 = Base36
static DEFAULT_ALPHABET: Mutex<Option<String>> = Mutex::new(None);

#[napi(object)]
#[derive(Default)]
pub struct ToStringOptions {
  pub enc: Option<String>,
  pub encoding: Option<String>,
  pub alphabet: Option<String>,
}

#[napi(object)]
#[derive(Default)]
pub struct CreateOptions {
  pub timestamp: Option<i64>,
  pub payload: Option<Uint8Array>,
  pub timestamp_precision: Option<String>,
}

fn convert_base(
  input: &[u8],
  in_base: u32,
  out_base: u32,
  out_len: usize,
) -> Result<Vec<u8>> {
  let mut out = vec![0u8; out_len];
  let word_len = 1;
  let word_base = in_base as u64;

  let in_len = input.len();
  let mut out_used = out_len.saturating_sub(1);

  let mut i = (in_len % word_len) as i32;
  if i > 0 {
    i -= word_len as i32;
  }

  while i < in_len as i32 {
    let mut carry: u64 = 0;
    let start = if i < 0 { 0 } else { i as usize };
    let end = (i + word_len as i32) as usize;
    for j in start..end {
      if j < in_len {
        carry = carry * (in_base as u64) + (input[j] as u64);
      }
    }

    for j in (0..out_len).rev() {
      carry += (out[j] as u64) * word_base;
      out[j] = (carry % (out_base as u64)) as u8;
      carry /= out_base as u64;

      if carry == 0 && j <= out_used {
        out_used = j;
        break;
      }
    }

    if carry != 0 {
      return Err(Error::new(
        Status::InvalidArg,
        "Output length too small for base conversion",
      ));
    }

    i += word_len as i32;
  }

  Ok(out)
}

fn encode_base36(bytes: &[u8; 20], alphabet: Option<&str>) -> Result<String> {
  let alpha_bytes = match alphabet {
    Some(a) => {
      let b = a.as_bytes();
      if b.len() != 36 {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Base36 alphabet must be 36 characters, got {}", b.len()),
        ));
      }
      b
    }
    None => DEFAULT_BASE36_ALPHABET,
  };

  // 20 bytes (160 bits) converted to Base36 requires 31 digits
  let digit_values = convert_base(bytes, 256, 36, 31)?;
  let mut out = Vec::with_capacity(31);
  for d in digit_values {
    out.push(alpha_bytes[d as usize]);
  }

  String::from_utf8(out).map_err(|e| Error::new(Status::GenericFailure, e.to_string()))
}

fn decode_base36(input: &str, alphabet: Option<&str>) -> Result<[u8; 20]> {
  let clean: String = input.chars().filter(|c| *c != '-').collect();
  if clean.len() != 31 {
    return Err(Error::new(
      Status::InvalidArg,
      format!("Invalid Base36 string length: expected 31 chars, got {}", clean.len()),
    ));
  }

  let custom_alpha = alphabet.map(|a| a.as_bytes());
  if let Some(a) = custom_alpha {
    if a.len() != 36 {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Base36 alphabet must be 36 characters, got {}", a.len()),
      ));
    }
  }

  let mut digit_values = Vec::with_capacity(31);
  for ch in clean.chars() {
    let val = match custom_alpha {
      Some(a) => {
        let ch_u8 = ch as u8;
        let mut found = None;
        for (idx, &b) in a.iter().enumerate() {
          if b == ch_u8 || b.to_ascii_lowercase() == ch_u8.to_ascii_lowercase() {
            found = Some(idx as u8);
            break;
          }
        }
        found.ok_or_else(|| {
          Error::new(
            Status::InvalidArg,
            format!("Character '{}' not found in custom alphabet", ch),
          )
        })?
      }
      None => {
        let ch_u8 = ch.to_ascii_lowercase() as u8;
        let mut found = None;
        for (idx, &b) in DEFAULT_BASE36_ALPHABET.iter().enumerate() {
          if b == ch_u8 {
            found = Some(idx as u8);
            break;
          }
        }
        found.ok_or_else(|| {
          Error::new(
            Status::InvalidArg,
            format!("Invalid Base36 character '{}'", ch),
          )
        })?
      }
    };
    digit_values.push(val);
  }

  let bytes_vec = convert_base(&digit_values, 36, 256, 20)?;
  let mut bytes = [0u8; 20];
  bytes.copy_from_slice(&bytes_vec);
  Ok(bytes)
}

fn get_5bit(bytes: &[u8; 20], chunk: usize) -> u8 {
  let bit_idx = chunk * 5;
  let byte_idx = bit_idx / 8;
  let bit_rem = bit_idx % 8;

  let mut val: u32 = (bytes[byte_idx] as u32) << 16;
  if byte_idx + 1 < 20 {
    val |= (bytes[byte_idx + 1] as u32) << 8;
  }
  if byte_idx + 2 < 20 {
    val |= bytes[byte_idx + 2] as u32;
  }

  let shift = 24 - 5 - bit_rem;
  ((val >> shift) & 0x1f) as u8
}

fn set_5bit(bytes: &mut [u8; 20], chunk: usize, val5: u8) {
  let val5 = val5 & 0x1f;
  let bit_idx = chunk * 5;
  let byte_idx = bit_idx / 8;
  let bit_rem = bit_idx % 8;

  let shift = 24 - 5 - bit_rem;
  let mask32: u32 = !(0x1f_u32 << shift);
  let val32: u32 = (val5 as u32) << shift;

  let mut cur32: u32 = (bytes[byte_idx] as u32) << 16;
  if byte_idx + 1 < 20 {
    cur32 |= (bytes[byte_idx + 1] as u32) << 8;
  }
  if byte_idx + 2 < 20 {
    cur32 |= bytes[byte_idx + 2] as u32;
  }

  cur32 = (cur32 & mask32) | val32;

  bytes[byte_idx] = (cur32 >> 16) as u8;
  if byte_idx + 1 < 20 {
    bytes[byte_idx + 1] = (cur32 >> 8) as u8;
  }
  if byte_idx + 2 < 20 {
    bytes[byte_idx + 2] = cur32 as u8;
  }
}

fn encode_crockford(bytes: &[u8; 20], alphabet: Option<&str>) -> Result<String> {
  let alpha_bytes = match alphabet {
    Some(a) => {
      let b = a.as_bytes();
      if b.len() != 32 {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Crockford Base32 alphabet must be 32 characters, got {}", b.len()),
        ));
      }
      b
    }
    None => DEFAULT_CROCKFORD_ALPHABET,
  };

  let mut out = Vec::with_capacity(32);
  for i in 0..32 {
    let five = get_5bit(bytes, i);
    out.push(alpha_bytes[five as usize]);
  }

  String::from_utf8(out).map_err(|e| Error::new(Status::GenericFailure, e.to_string()))
}

fn decode_crockford(input: &str, alphabet: Option<&str>) -> Result<[u8; 20]> {
  let clean: String = input.chars().filter(|c| *c != '-').collect();
  if clean.len() != 32 {
    return Err(Error::new(
      Status::InvalidArg,
      format!("Invalid Crockford Base32 string length: expected 32 chars, got {}", clean.len()),
    ));
  }

  let custom_alpha = alphabet.map(|a| a.as_bytes());
  if let Some(a) = custom_alpha {
    if a.len() != 32 {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Crockford Base32 alphabet must be 32 characters, got {}", a.len()),
      ));
    }
  }

  let mut bytes = [0u8; 20];
  for (chunk, ch) in clean.chars().enumerate() {
    let val = match custom_alpha {
      Some(a) => {
        let ch_u8 = ch as u8;
        let mut found = None;
        for (idx, &b) in a.iter().enumerate() {
          if b == ch_u8
            || b.to_ascii_uppercase() == ch_u8.to_ascii_uppercase() {
            found = Some(idx as u8);
            break;
          }
        }
        found.ok_or_else(|| {
          Error::new(
            Status::InvalidArg,
            format!("Character '{}' not found in custom alphabet", ch),
          )
        })?
      }
      None => {
        let norm = match ch {
          'O' | 'o' => '0',
          'I' | 'i' | 'L' | 'l' => '1',
          c => c.to_ascii_uppercase(),
        };
        let norm_u8 = norm as u8;
        let mut found = None;
        for (idx, &b) in DEFAULT_CROCKFORD_ALPHABET.iter().enumerate() {
          if b == norm_u8 {
            found = Some(idx as u8);
            break;
          }
        }
        found.ok_or_else(|| {
          Error::new(
            Status::InvalidArg,
            format!("Invalid Crockford Base32 character '{}'", ch),
          )
        })?
      }
    };
    set_5bit(&mut bytes, chunk, val);
  }

  Ok(bytes)
}

/// Utility function to shuffle an alphabet deterministically with an optional seed (or randomly if no seed is provided).
#[napi]
pub fn shuffle_alphabet(alphabet: String, seed: Option<String>) -> Result<String> {
  let mut chars: Vec<char> = alphabet.chars().collect();
  let len = chars.len();
  if len == 0 {
    return Ok(alphabet);
  }

  let mut state: u64 = match seed {
    Some(s) => {
      let mut h: u64 = 0xcbf29ce484222325;
      for b in s.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
      }
      h
    }
    None => std::time::SystemTime::now()
      .duration_since(std::time::UNIX_EPOCH)
      .map(|d| d.as_nanos() as u64)
      .unwrap_or(123456789),
  };

  let mut next_rnd = || {
    state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    state
  };

  for i in (1..len).rev() {
    let j = (next_rnd() as usize) % (i + 1);
    chars.swap(i, j);
  }

  Ok(chars.into_iter().collect())
}

#[napi]
pub struct Ksuid {
  inner: InnerKsuid,
}

#[napi]
impl Ksuid {
  /// Configure default toString encoding ("base62", "base32" / "crockford", or "base36") and optional custom alphabet.
  #[napi]
  pub fn set_default_encoding(encoding: String, alphabet: Option<String>) -> Result<()> {
    match encoding.to_lowercase().as_str() {
      "crockford" | "crockford_base32" | "base32" => {
        DEFAULT_ENCODING.store(1, Ordering::SeqCst);
        let mut guard = DEFAULT_ALPHABET.lock().map_err(|_| {
          Error::new(Status::GenericFailure, "Failed to lock default alphabet")
        })?;
        *guard = alphabet;
        Ok(())
      }
      "base36" => {
        DEFAULT_ENCODING.store(2, Ordering::SeqCst);
        let mut guard = DEFAULT_ALPHABET.lock().map_err(|_| {
          Error::new(Status::GenericFailure, "Failed to lock default alphabet")
        })?;
        *guard = alphabet;
        Ok(())
      }
      "base62" => {
        DEFAULT_ENCODING.store(0, Ordering::SeqCst);
        let mut guard = DEFAULT_ALPHABET.lock().map_err(|_| {
          Error::new(Status::GenericFailure, "Failed to lock default alphabet")
        })?;
        *guard = None;
        Ok(())
      }
      _ => Err(Error::new(
        Status::InvalidArg,
        format!("Unsupported encoding: {}. Choose 'base62', 'base32', or 'base36'.", encoding),
      )),
    }
  }

  /// Get current default toString encoding ("base62", "crockford", or "base36").
  #[napi]
  pub fn get_default_encoding() -> String {
    match DEFAULT_ENCODING.load(Ordering::SeqCst) {
      1 => "crockford".to_string(),
      2 => "base36".to_string(),
      _ => "base62".to_string(),
    }
  }

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
      (None, Some(p)) => InnerKsuid::from_seconds(None, Some(&p)),
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

  /// Create a Ksuid from Crockford Base32 encoded string with optional custom alphabet.
  #[napi(factory)]
  pub fn from_crockford_base32(str: String, alphabet: Option<String>) -> Result<Self> {
    let raw_bytes = decode_crockford(&str, alphabet.as_deref())?;
    let inner = InnerKsuid::from_bytes(raw_bytes);
    Ok(Ksuid { inner })
  }

  /// Create a Ksuid from Base36 encoded string with optional custom alphabet.
  #[napi(factory)]
  pub fn from_base36(str: String, alphabet: Option<String>) -> Result<Self> {
    let raw_bytes = decode_base36(&str, alphabet.as_deref())?;
    let inner = InnerKsuid::from_bytes(raw_bytes);
    Ok(Ksuid { inner })
  }

  /// String representation with optional encoding string or options object ({ enc: "base32" | "base36" | "base62", alphabet?: string }).
  #[napi]
  pub fn to_string(
    &self,
    options: Option<Either<String, ToStringOptions>>,
    alphabet: Option<String>,
  ) -> Result<String> {
    let (enc_opt, custom_alpha) = match options {
      Some(Either::A(enc_str)) => (Some(enc_str), alphabet),
      Some(Either::B(opts)) => {
        let enc = opts.enc.or(opts.encoding);
        let alpha = opts.alphabet.or(alphabet);
        (enc, alpha)
      }
      None => (None, alphabet),
    };

    let enc = match enc_opt {
      Some(e) => e,
      None => Self::get_default_encoding(),
    };

    match enc.to_lowercase().as_str() {
      "crockford" | "crockford_base32" | "base32" => {
        let alpha = match custom_alpha {
          Some(a) => Some(a),
          None => DEFAULT_ALPHABET.lock().unwrap().clone(),
        };
        self.to_crockford_base32(alpha)
      }
      "base36" => {
        let alpha = match custom_alpha {
          Some(a) => Some(a),
          None => DEFAULT_ALPHABET.lock().unwrap().clone(),
        };
        self.to_base36(alpha)
      }
      "base62" => Ok(self.inner.to_string()),
      _ => Err(Error::new(
        Status::InvalidArg,
        format!("Unsupported encoding: {}. Choose 'crockford', 'base32', 'base36', or 'base62'.", enc),
      )),
    }
  }

  /// Base62 string representation (explicit alias).
  #[napi]
  pub fn to_base62(&self) -> String {
    self.inner.to_base62()
  }

  /// Encode Ksuid to Crockford Base32 string with optional custom 32-character alphabet.
  #[napi]
  pub fn to_crockford_base32(&self, alphabet: Option<String>) -> Result<String> {
    encode_crockford(self.inner.bytes(), alphabet.as_deref())
  }

  /// Encode Ksuid to Base36 string with optional custom 36-character alphabet.
  #[napi]
  pub fn to_base36(&self, alphabet: Option<String>) -> Result<String> {
    encode_base36(self.inner.bytes(), alphabet.as_deref())
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

  /// Timestamp in milliseconds since UNIX epoch.
  #[napi]
  pub fn timestamp_ms(&self) -> i64 {
    self.inner.timestamp_seconds() * 1000
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

/// KsuidMs: 48-bit timestamp resolution (milliseconds since epoch) + 14-byte payload.
#[napi]
pub struct KsuidMs {
  inner: InnerKsuidMs,
}

#[napi]
impl KsuidMs {
  /// Create a new KsuidMs with an optional timestamp (in milliseconds since UNIX epoch) and optional 14-byte payload.
  #[napi(constructor)]
  pub fn new(timestamp_ms: Option<i64>, payload: Option<Uint8Array>) -> Result<Self> {
    let payload_bytes = match payload {
      Some(arr) => {
        let slice: &[u8] = arr.as_ref();
        if slice.len() != InnerKsuidMs::PAYLOAD_BYTES {
          return Err(Error::new(
            Status::InvalidArg,
            format!(
              "Payload for KsuidMs must be exactly {} bytes long, got {}",
              InnerKsuidMs::PAYLOAD_BYTES,
              slice.len()
            ),
          ));
        }
        let mut bytes = [0u8; InnerKsuidMs::PAYLOAD_BYTES];
        bytes.copy_from_slice(slice);
        Some(bytes)
      }
      None => None,
    };

    let inner = match (timestamp_ms, payload_bytes) {
      (Some(ts), Some(p)) => InnerKsuidMs::from_millis(Some(ts), Some(&p)),
      (Some(ts), None) => InnerKsuidMs::from_millis(Some(ts), None),
      (None, Some(p)) => InnerKsuidMs::from_millis(None, Some(&p)),
      (None, None) => InnerKsuidMs::from_millis(None, None),
    };

    Ok(KsuidMs { inner })
  }

  /// Timestamp is now in milliseconds, payload is randomly generated or provided.
  #[napi(factory)]
  pub fn now(payload: Option<Uint8Array>) -> Result<Self> {
    Self::new(None, payload)
  }

  /// Create KsuidMs from base62 string.
  #[napi(factory)]
  pub fn from_base62(base62: String) -> Result<Self> {
    let inner = InnerKsuidMs::from_base62(&base62).map_err(|e| {
      Error::new(
        Status::InvalidArg,
        format!("Invalid base62 KSUID-MS string: {}", e),
      )
    })?;
    Ok(KsuidMs { inner })
  }

  /// Create KsuidMs from 20 raw bytes.
  #[napi(factory)]
  pub fn from_bytes(bytes: Uint8Array) -> Result<Self> {
    let slice: &[u8] = bytes.as_ref();
    if slice.len() != 20 {
      return Err(Error::new(
        Status::InvalidArg,
        format!("KSUID-MS bytes must be exactly 20 bytes long, got {}", slice.len()),
      ));
    }
    let mut arr = [0u8; 20];
    arr.copy_from_slice(slice);
    let inner = InnerKsuidMs::from_bytes(arr);
    Ok(KsuidMs { inner })
  }

  /// Create a KsuidMs from Crockford Base32 encoded string with optional custom alphabet.
  #[napi(factory)]
  pub fn from_crockford_base32(str: String, alphabet: Option<String>) -> Result<Self> {
    let raw_bytes = decode_crockford(&str, alphabet.as_deref())?;
    let inner = InnerKsuidMs::from_bytes(raw_bytes);
    Ok(KsuidMs { inner })
  }

  /// Create a KsuidMs from Base36 encoded string with optional custom alphabet.
  #[napi(factory)]
  pub fn from_base36(str: String, alphabet: Option<String>) -> Result<Self> {
    let raw_bytes = decode_base36(&str, alphabet.as_deref())?;
    let inner = InnerKsuidMs::from_bytes(raw_bytes);
    Ok(KsuidMs { inner })
  }

  /// Base62 string representation.
  #[napi]
  pub fn to_string(
    &self,
    options: Option<Either<String, ToStringOptions>>,
    alphabet: Option<String>,
  ) -> Result<String> {
    let (enc_opt, custom_alpha) = match options {
      Some(Either::A(enc_str)) => (Some(enc_str), alphabet),
      Some(Either::B(opts)) => {
        let enc = opts.enc.or(opts.encoding);
        let alpha = opts.alphabet.or(alphabet);
        (enc, alpha)
      }
      None => (None, alphabet),
    };

    let enc = match enc_opt {
      Some(e) => e,
      None => Ksuid::get_default_encoding(),
    };

    match enc.to_lowercase().as_str() {
      "crockford" | "crockford_base32" | "base32" => {
        let alpha = match custom_alpha {
          Some(a) => Some(a),
          None => DEFAULT_ALPHABET.lock().unwrap().clone(),
        };
        self.to_crockford_base32(alpha)
      }
      "base36" => {
        let alpha = match custom_alpha {
          Some(a) => Some(a),
          None => DEFAULT_ALPHABET.lock().unwrap().clone(),
        };
        self.to_base36(alpha)
      }
      "base62" => Ok(self.inner.to_string()),
      _ => Err(Error::new(
        Status::InvalidArg,
        format!("Unsupported encoding: {}. Choose 'crockford', 'base32', 'base36', or 'base62'.", enc),
      )),
    }
  }

  /// Base62 string representation.
  #[napi]
  pub fn to_base62(&self) -> String {
    self.inner.to_base62()
  }

  /// Encode KsuidMs to Crockford Base32 string with optional custom 32-character alphabet.
  #[napi]
  pub fn to_crockford_base32(&self, alphabet: Option<String>) -> Result<String> {
    encode_crockford(self.inner.bytes(), alphabet.as_deref())
  }

  /// Encode KsuidMs to Base36 string with optional custom 36-character alphabet.
  #[napi]
  pub fn to_base36(&self, alphabet: Option<String>) -> Result<String> {
    encode_base36(self.inner.bytes(), alphabet.as_deref())
  }

  /// 20-byte slice representing the KsuidMs.
  #[napi]
  pub fn bytes(&self) -> Uint8Array {
    Uint8Array::from(&self.inner.bytes()[..])
  }

  /// 14-byte payload portion of the KsuidMs.
  #[napi]
  pub fn payload_bytes(&self) -> Uint8Array {
    Uint8Array::from(self.inner.payload())
  }

  /// Timestamp in milliseconds since UNIX epoch.
  #[napi]
  pub fn timestamp_ms(&self) -> i64 {
    self.inner.timestamp_millis()
  }

  /// Timestamp in seconds since UNIX epoch.
  #[napi]
  pub fn timestamp_seconds(&self) -> i64 {
    self.inner.timestamp_millis() / 1000
  }

  /// Compare two KsuidMs: returns -1 if self < other, 0 if equal, 1 if self > other.
  #[napi]
  pub fn compare_to(&self, other: &KsuidMs) -> i32 {
    match self.inner.cmp(&other.inner) {
      std::cmp::Ordering::Less => -1,
      std::cmp::Ordering::Equal => 0,
      std::cmp::Ordering::Greater => 1,
    }
  }

  /// Check equality with another KsuidMs instance.
  #[napi]
  pub fn equals(&self, other: &KsuidMs) -> bool {
    self.inner == other.inner
  }
}
