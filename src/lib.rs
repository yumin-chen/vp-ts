use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

const KSUID_EPOCH: u64 = 1_400_000_000; // 2014-05-13T16:53:20Z
const BASE62_ALPHABET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const CROCKFORD_BASE32_ALPHABET: &str = "0123456789ABCDEFGHJKMNPQRSTVWXYZ";

#[napi(object)]
#[derive(Clone, Debug, Default)]
pub struct KsuidOptions {
  pub enc: Option<String>,
  pub alphabet: Option<String>,
  pub timestamp_size: Option<String>,
}

fn get_now_ms() -> u64 {
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap_or_default()
    .as_millis() as u64
}

fn get_now_sec() -> u64 {
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap_or_default()
    .as_secs()
}

fn random_payload(len: usize) -> Vec<u8> {
  let mut buf = vec![0u8; len];
  getrandom::getrandom(&mut buf).unwrap_or_default();
  buf
}

fn encode_with_alphabet(bytes: &[u8], alphabet: &str, expected_len: usize) -> String {
  let chars: Vec<char> = alphabet.chars().collect();
  let radix = chars.len();
  if radix == 0 {
    return String::new();
  }

  let mut num: Vec<u8> = bytes.to_vec();
  let mut digits = Vec::new();

  while num.iter().any(|&b| b != 0) {
    let mut remainder = 0u64;
    for byte in num.iter_mut() {
      let cur = (remainder << 8) | (*byte as u64);
      *byte = (cur / radix as u64) as u8;
      remainder = cur % radix as u64;
    }
    digits.push(chars[remainder as usize]);
  }

  while digits.len() < expected_len {
    digits.push(chars[0]);
  }

  digits.reverse();
  digits.into_iter().collect()
}

fn decode_with_alphabet(encoded: &str, alphabet: &str, target_len: usize) -> Result<Vec<u8>> {
  let chars: Vec<char> = alphabet.chars().collect();
  let radix = chars.len();
  if radix == 0 {
    return Err(Error::new(Status::InvalidArg, "Alphabet cannot be empty"));
  }

  let mut map = HashMap::new();
  for (idx, &ch) in chars.iter().enumerate() {
    map.insert(ch, idx as u64);
  }

  if alphabet.to_ascii_uppercase() == CROCKFORD_BASE32_ALPHABET {
    for (idx, &ch) in chars.iter().enumerate() {
      map.insert(ch.to_ascii_lowercase(), idx as u64);
      map.insert(ch.to_ascii_uppercase(), idx as u64);
    }
    map.insert('i', 1);
    map.insert('I', 1);
    map.insert('l', 1);
    map.insert('L', 1);
    map.insert('o', 0);
    map.insert('O', 0);
  }

  let mut result = vec![0u8; target_len];

  for ch in encoded.chars() {
    let digit = match map.get(&ch) {
      Some(&d) => d,
      None => {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Invalid character '{}' for alphabet", ch),
        ));
      }
    };

    let mut carry = digit;
    for byte in result.iter_mut().rev() {
      let cur = (*byte as u64) * (radix as u64) + carry;
      *byte = (cur & 0xff) as u8;
      carry = cur >> 8;
    }
    if carry > 0 {
      return Err(Error::new(
        Status::InvalidArg,
        "Value overflowed target length",
      ));
    }
  }

  Ok(result)
}

#[napi]
pub struct Ksuid {
  bytes: [u8; 20],
  enc: String,
  alphabet: Option<String>,
  timestamp_size: String,
}

#[napi]
impl Ksuid {
  #[napi(constructor)]
  pub fn new(
    timestamp: Option<i64>,
    payload: Option<Buffer>,
    options: Option<KsuidOptions>,
  ) -> Result<Self> {
    let opts = options.unwrap_or_default();
    let enc = opts.enc.unwrap_or_else(|| "base62".to_string());
    let alphabet = opts.alphabet;
    let ts_size = opts.timestamp_size.unwrap_or_else(|| "32bit".to_string());

    let mut raw = [0u8; 20];

    match ts_size.as_str() {
      "32bit" => {
        let ts_sec = match timestamp {
          Some(t) => t as u64,
          None => get_now_sec(),
        };
        let ksuid_ts = if ts_sec >= KSUID_EPOCH {
          (ts_sec - KSUID_EPOCH) as u32
        } else {
          ts_sec as u32
        };
        raw[0..4].copy_from_slice(&ksuid_ts.to_be_bytes());

        let p_bytes = match payload {
          Some(buf) => {
            let mut p = vec![0u8; 16];
            let copy_len = buf.len().min(16);
            p[..copy_len].copy_from_slice(&buf[..copy_len]);
            p
          }
          None => random_payload(16),
        };
        raw[4..20].copy_from_slice(&p_bytes);
      }
      "48bit" => {
        let ts_ms = match timestamp {
          Some(t) => t as u64,
          None => get_now_ms(),
        };
        let be = ts_ms.to_be_bytes();
        raw[0..6].copy_from_slice(&be[2..8]);

        let p_bytes = match payload {
          Some(buf) => {
            let mut p = vec![0u8; 14];
            let copy_len = buf.len().min(14);
            p[..copy_len].copy_from_slice(&buf[..copy_len]);
            p
          }
          None => random_payload(14),
        };
        raw[6..20].copy_from_slice(&p_bytes);
      }
      "64bit" => {
        let ts_ms = match timestamp {
          Some(t) => t as u64,
          None => get_now_ms(),
        };
        raw[0..8].copy_from_slice(&ts_ms.to_be_bytes());

        let p_bytes = match payload {
          Some(buf) => {
            let mut p = vec![0u8; 12];
            let copy_len = buf.len().min(12);
            p[..copy_len].copy_from_slice(&buf[..copy_len]);
            p
          }
          None => random_payload(12),
        };
        raw[8..20].copy_from_slice(&p_bytes);
      }
      other => {
        return Err(Error::new(
          Status::InvalidArg,
          format!(
            "Unsupported timestampSize '{}'. Must be '32bit', '48bit', or '64bit'",
            other
          ),
        ));
      }
    }

    Ok(Self {
      bytes: raw,
      enc,
      alphabet,
      timestamp_size: ts_size,
    })
  }

  #[napi(factory)]
  pub fn now(options: Option<KsuidOptions>) -> Result<Self> {
    Self::new(None, None, options)
  }

  #[napi(factory)]
  pub fn from_seconds(
    timestamp_seconds: Option<i64>,
    payload: Option<Buffer>,
    options: Option<KsuidOptions>,
  ) -> Result<Self> {
    let mut opts = options.unwrap_or_default();
    if opts.timestamp_size.is_none() {
      opts.timestamp_size = Some("32bit".to_string());
    }
    Self::new(timestamp_seconds, payload, Some(opts))
  }

  #[napi(factory)]
  pub fn from_millis(
    timestamp_ms: Option<i64>,
    payload: Option<Buffer>,
    options: Option<KsuidOptions>,
  ) -> Result<Self> {
    let mut opts = options.unwrap_or_default();
    if opts.timestamp_size.is_none() {
      opts.timestamp_size = Some("48bit".to_string());
    }
    Self::new(timestamp_ms, payload, Some(opts))
  }

  #[napi(factory)]
  pub fn from_base62(base62: String, options: Option<KsuidOptions>) -> Result<Self> {
    let opts = options.unwrap_or_default();
    let alphabet = opts
      .alphabet
      .clone()
      .unwrap_or_else(|| BASE62_ALPHABET.to_string());
    let decoded = decode_with_alphabet(&base62, &alphabet, 20)?;
    let mut arr = [0u8; 20];
    arr.copy_from_slice(&decoded);

    let mut res_opts = opts;
    res_opts.enc = Some("base62".to_string());
    Self::from_bytes(Buffer::from(arr.as_slice()), Some(res_opts))
  }

  #[napi(factory)]
  pub fn from_base32(base32: String, options: Option<KsuidOptions>) -> Result<Self> {
    let opts = options.unwrap_or_default();
    let alphabet = opts
      .alphabet
      .clone()
      .unwrap_or_else(|| CROCKFORD_BASE32_ALPHABET.to_string());
    let decoded = decode_with_alphabet(&base32, &alphabet, 20)?;
    let mut arr = [0u8; 20];
    arr.copy_from_slice(&decoded);

    let mut res_opts = opts;
    res_opts.enc = Some("base32".to_string());
    Self::from_bytes(Buffer::from(arr.as_slice()), Some(res_opts))
  }

  #[napi(factory)]
  pub fn from_str(str: String, options: Option<KsuidOptions>) -> Result<Self> {
    let opts = options.unwrap_or_default();
    let enc = opts
      .enc
      .as_deref()
      .unwrap_or(if str.len() == 32 { "base32" } else { "base62" });
    if enc == "base32" {
      Self::from_base32(str, Some(opts))
    } else {
      Self::from_base62(str, Some(opts))
    }
  }

  #[napi(factory)]
  pub fn from_bytes(bytes: Buffer, options: Option<KsuidOptions>) -> Result<Self> {
    if bytes.len() != 20 {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Ksuid bytes length must be 20, got {}", bytes.len()),
      ));
    }
    let opts = options.unwrap_or_default();
    let mut arr = [0u8; 20];
    arr.copy_from_slice(&bytes);
    Ok(Self {
      bytes: arr,
      enc: opts.enc.unwrap_or_else(|| "base62".to_string()),
      alphabet: opts.alphabet,
      timestamp_size: opts.timestamp_size.unwrap_or_else(|| "32bit".to_string()),
    })
  }

  #[napi]
  pub fn to_string(&self) -> String {
    if self.enc == "base32" {
      self.to_base32()
    } else {
      self.to_base62()
    }
  }

  #[napi]
  pub fn to_base62(&self) -> String {
    let alphabet = self.alphabet.as_deref().unwrap_or(BASE62_ALPHABET);
    encode_with_alphabet(&self.bytes, alphabet, 27)
  }

  #[napi]
  pub fn to_base32(&self) -> String {
    let alphabet = self
      .alphabet
      .as_deref()
      .unwrap_or(CROCKFORD_BASE32_ALPHABET);
    encode_with_alphabet(&self.bytes, alphabet, 32)
  }

  #[napi]
  pub fn bytes(&self) -> Buffer {
    Buffer::from(self.bytes.as_slice())
  }

  #[napi]
  pub fn payload(&self) -> Buffer {
    match self.timestamp_size.as_str() {
      "32bit" => Buffer::from(&self.bytes[4..20]),
      "48bit" => Buffer::from(&self.bytes[6..20]),
      "64bit" => Buffer::from(&self.bytes[8..20]),
      _ => Buffer::from(&self.bytes[4..20]),
    }
  }

  #[napi]
  pub fn timestamp_seconds(&self) -> i64 {
    match self.timestamp_size.as_str() {
      "32bit" => {
        let raw_ts =
          u32::from_be_bytes([self.bytes[0], self.bytes[1], self.bytes[2], self.bytes[3]]) as u64;
        (raw_ts + KSUID_EPOCH) as i64
      }
      "48bit" => self.timestamp_ms() / 1000,
      "64bit" => self.timestamp_ms() / 1000,
      _ => 0,
    }
  }

  #[napi]
  pub fn timestamp_ms(&self) -> i64 {
    match self.timestamp_size.as_str() {
      "32bit" => self.timestamp_seconds() * 1000,
      "48bit" => {
        let mut be = [0u8; 8];
        be[2..8].copy_from_slice(&self.bytes[0..6]);
        u64::from_be_bytes(be) as i64
      }
      "64bit" => u64::from_be_bytes([
        self.bytes[0],
        self.bytes[1],
        self.bytes[2],
        self.bytes[3],
        self.bytes[4],
        self.bytes[5],
        self.bytes[6],
        self.bytes[7],
      ]) as i64,
      _ => 0,
    }
  }

  #[napi]
  pub fn compare(&self, other: &Ksuid) -> i32 {
    match self.bytes.cmp(&other.bytes) {
      std::cmp::Ordering::Less => -1,
      std::cmp::Ordering::Equal => 0,
      std::cmp::Ordering::Greater => 1,
    }
  }

  #[napi]
  pub fn equals(&self, other: &Ksuid) -> bool {
    self.bytes == other.bytes
  }
}

#[napi]
pub struct KsuidMs {
  inner: Ksuid,
}

#[napi]
impl KsuidMs {
  #[napi(constructor)]
  pub fn new(
    timestamp_ms: Option<i64>,
    payload: Option<Buffer>,
    options: Option<KsuidOptions>,
  ) -> Result<Self> {
    let mut opts = options.unwrap_or_default();
    if opts.timestamp_size.is_none() {
      opts.timestamp_size = Some("64bit".to_string());
    }
    let inner = Ksuid::new(timestamp_ms, payload, Some(opts))?;
    Ok(Self { inner })
  }

  #[napi(factory)]
  pub fn now(options: Option<KsuidOptions>) -> Result<Self> {
    Self::new(None, None, options)
  }

  #[napi(factory)]
  pub fn from_millis(
    timestamp_ms: Option<i64>,
    payload: Option<Buffer>,
    options: Option<KsuidOptions>,
  ) -> Result<Self> {
    Self::new(timestamp_ms, payload, options)
  }

  #[napi(factory)]
  pub fn from_base62(base62: String, options: Option<KsuidOptions>) -> Result<Self> {
    let mut opts = options.unwrap_or_default();
    if opts.timestamp_size.is_none() {
      opts.timestamp_size = Some("64bit".to_string());
    }
    let inner = Ksuid::from_base62(base62, Some(opts))?;
    Ok(Self { inner })
  }

  #[napi(factory)]
  pub fn from_base32(base32: String, options: Option<KsuidOptions>) -> Result<Self> {
    let mut opts = options.unwrap_or_default();
    if opts.timestamp_size.is_none() {
      opts.timestamp_size = Some("64bit".to_string());
    }
    let inner = Ksuid::from_base32(base32, Some(opts))?;
    Ok(Self { inner })
  }

  #[napi(factory)]
  pub fn from_str(str: String, options: Option<KsuidOptions>) -> Result<Self> {
    let mut opts = options.unwrap_or_default();
    if opts.timestamp_size.is_none() {
      opts.timestamp_size = Some("64bit".to_string());
    }
    let inner = Ksuid::from_str(str, Some(opts))?;
    Ok(Self { inner })
  }

  #[napi(factory)]
  pub fn from_bytes(bytes: Buffer, options: Option<KsuidOptions>) -> Result<Self> {
    let mut opts = options.unwrap_or_default();
    if opts.timestamp_size.is_none() {
      opts.timestamp_size = Some("64bit".to_string());
    }
    let inner = Ksuid::from_bytes(bytes, Some(opts))?;
    Ok(Self { inner })
  }

  #[napi]
  pub fn to_string(&self) -> String {
    self.inner.to_string()
  }

  #[napi]
  pub fn to_base62(&self) -> String {
    self.inner.to_base62()
  }

  #[napi]
  pub fn to_base32(&self) -> String {
    self.inner.to_base32()
  }

  #[napi]
  pub fn bytes(&self) -> Buffer {
    self.inner.bytes()
  }

  #[napi]
  pub fn payload(&self) -> Buffer {
    self.inner.payload()
  }

  #[napi]
  pub fn timestamp_ms(&self) -> i64 {
    self.inner.timestamp_ms()
  }

  #[napi]
  pub fn compare(&self, other: &KsuidMs) -> i32 {
    self.inner.compare(&other.inner)
  }

  #[napi]
  pub fn equals(&self, other: &KsuidMs) -> bool {
    self.inner.equals(&other.inner)
  }
}
