use napi::bindgen_prelude::*;
use napi_derive::napi;
use svix_ksuid::{Ksuid as SvixKsuid, KsuidLike, KsuidMs as SvixKsuidMs};

const CROCKFORD_ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

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

pub fn encode_bytes_base32(bytes: &[u8], alphabet: &[u8]) -> String {
  let mut result = String::new();
  let mut bit_buf: u64 = 0;
  let mut bit_count = 0;

  for &b in bytes {
    bit_buf = (bit_buf << 8) | (b as u64);
    bit_count += 8;
    while bit_count >= 5 {
      bit_count -= 5;
      let idx = ((bit_buf >> bit_count) & 0x1F) as usize;
      result.push(alphabet[idx] as char);
    }
  }
  if bit_count > 0 {
    let idx = ((bit_buf << (5 - bit_count)) & 0x1F) as usize;
    result.push(alphabet[idx] as char);
  }
  result
}

pub fn decode_bytes_base32(s: &str, alphabet: &[u8]) -> Result<Vec<u8>> {
  let mut map = [255u8; 256];
  for (i, &ch) in alphabet.iter().enumerate() {
    map[ch as usize] = i as u8;
    if ch >= b'A' && ch <= b'Z' {
      map[(ch + 32) as usize] = i as u8;
    }
  }
  if alphabet == CROCKFORD_ALPHABET {
    map[b'O' as usize] = 0;
    map[b'o' as usize] = 0;
    map[b'I' as usize] = 1;
    map[b'i' as usize] = 1;
    map[b'L' as usize] = 1;
    map[b'l' as usize] = 1;
  }

  let mut bytes = Vec::new();
  let mut bit_buf: u64 = 0;
  let mut bit_count = 0;

  for &b in s.as_bytes() {
    let val = map[b as usize];
    if val == 255 {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Invalid character '{}' in base32 string", b as char),
      ));
    }
    bit_buf = (bit_buf << 5) | (val as u64);
    bit_count += 5;
    if bit_count >= 8 {
      bit_count -= 8;
      bytes.push(((bit_buf >> bit_count) & 0xFF) as u8);
    }
  }
  Ok(bytes)
}

#[napi(object)]
#[derive(Default, Clone)]
pub struct KsuidOptions {
  pub enc: Option<String>,
  pub alphabet: Option<String>,
  pub timestamp_size: Option<String>,
}

#[napi]
#[derive(Clone, PartialEq, Eq)]
pub struct Ksuid {
  inner: SvixKsuid,
  enc: String,
  alphabet: Option<String>,
  timestamp_size: String,
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
  pub fn now(payload: Option<Uint8Array>, options: Option<KsuidOptions>) -> Result<Self> {
    let p = parse_payload(payload)?;
    let opts = options.unwrap_or_default();
    Ok(Self {
      inner: SvixKsuid::now(p.as_deref()),
      enc: opts.enc.unwrap_or_else(|| "base62".to_string()),
      alphabet: opts.alphabet,
      timestamp_size: opts.timestamp_size.unwrap_or_else(|| "32bit".to_string()),
    })
  }

  #[napi]
  pub fn new(
    timestamp: Option<f64>,
    payload: Option<Uint8Array>,
    options: Option<KsuidOptions>,
  ) -> Result<Self> {
    let ts = parse_timestamp(timestamp)?;
    let p = parse_payload(payload)?;
    let opts = options.unwrap_or_default();
    Ok(Self {
      inner: SvixKsuid::new(ts, p.as_deref()),
      enc: opts.enc.unwrap_or_else(|| "base62".to_string()),
      alphabet: opts.alphabet,
      timestamp_size: opts.timestamp_size.unwrap_or_else(|| "32bit".to_string()),
    })
  }

  #[napi(js_name = "fromBase62")]
  pub fn from_base62(base62: String, options: Option<KsuidOptions>) -> Result<Self> {
    let inner = SvixKsuid::from_base62(&base62)
      .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
    let opts = options.unwrap_or_default();
    Ok(Self {
      inner,
      enc: opts.enc.unwrap_or_else(|| "base62".to_string()),
      alphabet: opts.alphabet,
      timestamp_size: opts.timestamp_size.unwrap_or_else(|| "32bit".to_string()),
    })
  }

  #[napi(js_name = "from_base62")]
  pub fn from_base62_snake(base62: String, options: Option<KsuidOptions>) -> Result<Self> {
    Self::from_base62(base62, options)
  }

  #[napi(js_name = "fromBase32")]
  pub fn from_base32(base32: String, options: Option<KsuidOptions>) -> Result<Self> {
    let opts = options.unwrap_or_default();
    let alpha = opts
      .alphabet
      .as_deref()
      .map(|s| s.as_bytes())
      .unwrap_or(CROCKFORD_ALPHABET);

    let bytes = decode_bytes_base32(&base32, alpha)?;
    if bytes.len() != 20 {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Base32 decoded length must be 20 bytes, got {}", bytes.len()),
      ));
    }
    let mut arr = [0u8; 20];
    arr.copy_from_slice(&bytes);
    Ok(Self {
      inner: SvixKsuid::from_bytes(arr),
      enc: "base32".to_string(),
      alphabet: opts.alphabet,
      timestamp_size: opts.timestamp_size.unwrap_or_else(|| "32bit".to_string()),
    })
  }

  #[napi(js_name = "from_base32")]
  pub fn from_base32_snake(base32: String, options: Option<KsuidOptions>) -> Result<Self> {
    Self::from_base32(base32, options)
  }

  #[napi(js_name = "fromStr")]
  pub fn from_str_js(s: String, options: Option<KsuidOptions>) -> Result<Self> {
    let opts = options.clone().unwrap_or_default();
    if opts.enc.as_deref() == Some("base32") {
      Self::from_base32(s, options)
    } else {
      Self::from_base62(s, options)
    }
  }

  #[napi(js_name = "from_str")]
  pub fn from_str_snake(s: String, options: Option<KsuidOptions>) -> Result<Self> {
    Self::from_str_js(s, options)
  }

  #[napi(js_name = "fromBytes")]
  pub fn from_bytes(bytes: Uint8Array, options: Option<KsuidOptions>) -> Result<Self> {
    let slice = bytes.as_ref();
    if slice.len() != 20 {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Bytes must be exactly 20 bytes, got {}", slice.len()),
      ));
    }
    let mut arr = [0u8; 20];
    arr.copy_from_slice(slice);
    let opts = options.unwrap_or_default();
    Ok(Self {
      inner: SvixKsuid::from_bytes(arr),
      enc: opts.enc.unwrap_or_else(|| "base62".to_string()),
      alphabet: opts.alphabet,
      timestamp_size: opts.timestamp_size.unwrap_or_else(|| "32bit".to_string()),
    })
  }

  #[napi(js_name = "from_bytes")]
  pub fn from_bytes_snake(bytes: Uint8Array, options: Option<KsuidOptions>) -> Result<Self> {
    Self::from_bytes(bytes, options)
  }

  #[napi(js_name = "fromSeconds")]
  pub fn from_seconds(
    seconds: Option<f64>,
    payload: Option<Uint8Array>,
    options: Option<KsuidOptions>,
  ) -> Result<Self> {
    let sec = seconds.map(|s| s as i64);
    let p = parse_payload(payload)?;
    let opts = options.unwrap_or_default();
    Ok(Self {
      inner: SvixKsuid::from_seconds(sec, p.as_deref()),
      enc: opts.enc.unwrap_or_else(|| "base62".to_string()),
      alphabet: opts.alphabet,
      timestamp_size: opts.timestamp_size.unwrap_or_else(|| "32bit".to_string()),
    })
  }

  #[napi(js_name = "from_seconds")]
  pub fn from_seconds_snake(
    seconds: Option<f64>,
    payload: Option<Uint8Array>,
    options: Option<KsuidOptions>,
  ) -> Result<Self> {
    Self::from_seconds(seconds, payload, options)
  }

  #[napi(js_name = "toBase62")]
  pub fn to_base62(&self) -> String {
    self.inner.to_base62()
  }

  #[napi(js_name = "to_base62")]
  pub fn to_base62_snake(&self) -> String {
    self.inner.to_base62()
  }

  #[napi(js_name = "toBase32")]
  pub fn to_base32(&self) -> String {
    let alpha = self
      .alphabet
      .as_deref()
      .map(|s| s.as_bytes())
      .unwrap_or(CROCKFORD_ALPHABET);
    encode_bytes_base32(self.inner.bytes().as_slice(), alpha)
  }

  #[napi(js_name = "to_base32")]
  pub fn to_base32_snake(&self) -> String {
    self.to_base32()
  }

  #[napi(js_name = "toString")]
  pub fn to_string_js(&self) -> String {
    if self.enc == "base32" {
      self.to_base32()
    } else {
      self.inner.to_string()
    }
  }

  #[napi(js_name = "to_string")]
  pub fn to_string_snake(&self) -> String {
    self.to_string_js()
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
#[derive(Clone, PartialEq, Eq)]
pub struct KsuidMs {
  inner: SvixKsuidMs,
  enc: String,
  alphabet: Option<String>,
  timestamp_size: String,
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
  pub fn now(payload: Option<Uint8Array>, options: Option<KsuidOptions>) -> Result<Self> {
    let p = parse_payload(payload)?;
    let opts = options.unwrap_or_default();
    Ok(Self {
      inner: SvixKsuidMs::now(p.as_deref()),
      enc: opts.enc.unwrap_or_else(|| "base62".to_string()),
      alphabet: opts.alphabet,
      timestamp_size: opts.timestamp_size.unwrap_or_else(|| "64bit".to_string()),
    })
  }

  #[napi]
  pub fn new(
    timestamp: Option<f64>,
    payload: Option<Uint8Array>,
    options: Option<KsuidOptions>,
  ) -> Result<Self> {
    let ts = parse_timestamp(timestamp)?;
    let p = parse_payload(payload)?;
    let opts = options.unwrap_or_default();
    Ok(Self {
      inner: SvixKsuidMs::new(ts, p.as_deref()),
      enc: opts.enc.unwrap_or_else(|| "base62".to_string()),
      alphabet: opts.alphabet,
      timestamp_size: opts.timestamp_size.unwrap_or_else(|| "64bit".to_string()),
    })
  }

  #[napi(js_name = "fromBase62")]
  pub fn from_base62(base62: String, options: Option<KsuidOptions>) -> Result<Self> {
    let inner = SvixKsuidMs::from_base62(&base62)
      .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
    let opts = options.unwrap_or_default();
    Ok(Self {
      inner,
      enc: opts.enc.unwrap_or_else(|| "base62".to_string()),
      alphabet: opts.alphabet,
      timestamp_size: opts.timestamp_size.unwrap_or_else(|| "64bit".to_string()),
    })
  }

  #[napi(js_name = "from_base62")]
  pub fn from_base62_snake(base62: String, options: Option<KsuidOptions>) -> Result<Self> {
    Self::from_base62(base62, options)
  }

  #[napi(js_name = "fromBase32")]
  pub fn from_base32(base32: String, options: Option<KsuidOptions>) -> Result<Self> {
    let opts = options.unwrap_or_default();
    let alpha = opts
      .alphabet
      .as_deref()
      .map(|s| s.as_bytes())
      .unwrap_or(CROCKFORD_ALPHABET);

    let bytes = decode_bytes_base32(&base32, alpha)?;
    if bytes.len() != 20 {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Base32 decoded length must be 20 bytes, got {}", bytes.len()),
      ));
    }
    let mut arr = [0u8; 20];
    arr.copy_from_slice(&bytes);
    Ok(Self {
      inner: SvixKsuidMs::from_bytes(arr),
      enc: "base32".to_string(),
      alphabet: opts.alphabet,
      timestamp_size: opts.timestamp_size.unwrap_or_else(|| "64bit".to_string()),
    })
  }

  #[napi(js_name = "from_base32")]
  pub fn from_base32_snake(base32: String, options: Option<KsuidOptions>) -> Result<Self> {
    Self::from_base32(base32, options)
  }

  #[napi(js_name = "fromStr")]
  pub fn from_str_js(s: String, options: Option<KsuidOptions>) -> Result<Self> {
    let opts = options.clone().unwrap_or_default();
    if opts.enc.as_deref() == Some("base32") {
      Self::from_base32(s, options)
    } else {
      Self::from_base62(s, options)
    }
  }

  #[napi(js_name = "from_str")]
  pub fn from_str_snake(s: String, options: Option<KsuidOptions>) -> Result<Self> {
    Self::from_str_js(s, options)
  }

  #[napi(js_name = "fromBytes")]
  pub fn from_bytes(bytes: Uint8Array, options: Option<KsuidOptions>) -> Result<Self> {
    let slice = bytes.as_ref();
    if slice.len() != 20 {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Bytes must be exactly 20 bytes, got {}", slice.len()),
      ));
    }
    let mut arr = [0u8; 20];
    arr.copy_from_slice(slice);
    let opts = options.unwrap_or_default();
    Ok(Self {
      inner: SvixKsuidMs::from_bytes(arr),
      enc: opts.enc.unwrap_or_else(|| "base62".to_string()),
      alphabet: opts.alphabet,
      timestamp_size: opts.timestamp_size.unwrap_or_else(|| "64bit".to_string()),
    })
  }

  #[napi(js_name = "from_bytes")]
  pub fn from_bytes_snake(bytes: Uint8Array, options: Option<KsuidOptions>) -> Result<Self> {
    Self::from_bytes(bytes, options)
  }

  #[napi(js_name = "fromMillis")]
  pub fn from_millis(
    millis: Option<f64>,
    payload: Option<Uint8Array>,
    options: Option<KsuidOptions>,
  ) -> Result<Self> {
    let ms = millis.map(|m| m as i64);
    let p = parse_payload(payload)?;
    let opts = options.unwrap_or_default();
    Ok(Self {
      inner: SvixKsuidMs::from_millis(ms, p.as_deref()),
      enc: opts.enc.unwrap_or_else(|| "base62".to_string()),
      alphabet: opts.alphabet,
      timestamp_size: opts.timestamp_size.unwrap_or_else(|| "64bit".to_string()),
    })
  }

  #[napi(js_name = "from_millis")]
  pub fn from_millis_snake(
    millis: Option<f64>,
    payload: Option<Uint8Array>,
    options: Option<KsuidOptions>,
  ) -> Result<Self> {
    Self::from_millis(millis, payload, options)
  }

  #[napi(js_name = "fromSeconds")]
  pub fn from_seconds(
    seconds: Option<f64>,
    payload: Option<Uint8Array>,
    options: Option<KsuidOptions>,
  ) -> Result<Self> {
    let sec = seconds.map(|s| (s * 1000.0) as i64);
    let p = parse_payload(payload)?;
    let opts = options.unwrap_or_default();
    Ok(Self {
      inner: SvixKsuidMs::from_millis(sec, p.as_deref()),
      enc: opts.enc.unwrap_or_else(|| "base62".to_string()),
      alphabet: opts.alphabet,
      timestamp_size: opts.timestamp_size.unwrap_or_else(|| "64bit".to_string()),
    })
  }

  #[napi(js_name = "from_seconds")]
  pub fn from_seconds_snake(
    seconds: Option<f64>,
    payload: Option<Uint8Array>,
    options: Option<KsuidOptions>,
  ) -> Result<Self> {
    Self::from_seconds(seconds, payload, options)
  }

  #[napi(js_name = "toBase62")]
  pub fn to_base62(&self) -> String {
    self.inner.to_base62()
  }

  #[napi(js_name = "to_base62")]
  pub fn to_base62_snake(&self) -> String {
    self.inner.to_base62()
  }

  #[napi(js_name = "toBase32")]
  pub fn to_base32(&self) -> String {
    let alpha = self
      .alphabet
      .as_deref()
      .map(|s| s.as_bytes())
      .unwrap_or(CROCKFORD_ALPHABET);
    encode_bytes_base32(self.inner.bytes().as_slice(), alpha)
  }

  #[napi(js_name = "to_base32")]
  pub fn to_base32_snake(&self) -> String {
    self.to_base32()
  }

  #[napi(js_name = "toString")]
  pub fn to_string_js(&self) -> String {
    if self.enc == "base32" {
      self.to_base32()
    } else {
      self.inner.to_string()
    }
  }

  #[napi(js_name = "to_string")]
  pub fn to_string_snake(&self) -> String {
    self.to_string_js()
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

#[napi]
pub fn encode_crockford(value: f64) -> String {
  let val = value as u64;
  let mut fits = Vec::with_capacity(13);
  encode_into_crockford(val, &mut fits);
  unsafe { String::from_utf8_unchecked(fits) }
}

fn encode_into_crockford(mut n: u64, w: &mut Vec<u8>) {
  const QUAD_SHIFT: usize = 60;
  const QUAD_RESET: usize = 4;
  const FIVE_SHIFT: usize = 59;
  const FIVE_RESET: usize = 5;
  const STOP_BIT: u64 = 1 << QUAD_SHIFT;

  if n == 0 {
    w.push(b'0');
    return;
  }

  match (n >> QUAD_SHIFT) as usize {
    0 => {
      n <<= QUAD_RESET;
      n |= 1;
      n <<= n.leading_zeros() / 5 * 5;
    }
    i => {
      n <<= QUAD_RESET;
      n |= 1;
      w.push(CROCKFORD_ALPHABET[i]);
    }
  }

  while n != STOP_BIT {
    w.push(CROCKFORD_ALPHABET[(n >> FIVE_SHIFT) as usize]);
    n <<= FIVE_RESET;
  }
}

#[napi]
pub fn decode_crockford(input: String) -> Result<f64> {
  if input.is_empty() {
    return Err(Error::new(Status::InvalidArg, "Empty string"));
  }
  let mut map = [-1i8; 256];
  for (i, &ch) in CROCKFORD_ALPHABET.iter().enumerate() {
    map[ch as usize] = i as i8;
    if ch >= b'A' && ch <= b'Z' {
      map[(ch + 32) as usize] = i as i8;
    }
  }
  map[b'O' as usize] = 0;
  map[b'o' as usize] = 0;
  map[b'I' as usize] = 1;
  map[b'i' as usize] = 1;
  map[b'L' as usize] = 1;
  map[b'l' as usize] = 1;

  let mut place: u64 = 32u64.pow(input.len() as u32 - 1);
  let mut n: u64 = 0;

  for u in input.bytes() {
    let digit = map[u as usize];
    if digit < 0 {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Invalid digit in crockford base32: {}", u as char),
      ));
    }
    n = n
      .checked_add((digit as u64).checked_mul(place).ok_or_else(|| {
        Error::new(Status::InvalidArg, "Overflow decoding crockford base32")
      })?)
      .ok_or_else(|| Error::new(Status::InvalidArg, "Overflow decoding crockford base32"))?;
    place >>= 5;
  }

  Ok(n as f64)
}
