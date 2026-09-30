use napi::bindgen_prelude::*;
use napi_derive::napi;
use svix_ksuid::{Ksuid as SvixKsuid, KsuidLike};

pub const DEFAULT_CROCKFORD_ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

fn get_alphabet(custom: Option<String>) -> Result<(Vec<u8>, bool)> {
  match custom {
    Some(s) => {
      let mut bytes = s.into_bytes();
      if bytes.len() < 32 {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Custom alphabet must be at least 32 characters, got {}", bytes.len()),
        ));
      }
      if bytes.len() > 32 {
        bytes.truncate(32);
      }
      Ok((bytes, false))
    }
    None => Ok((DEFAULT_CROCKFORD_ALPHABET.to_vec(), true)),
  }
}

fn normalize_char(c: char) -> char {
  match c {
    'o' | 'O' => '0',
    'i' | 'I' | 'l' | 'L' => '1',
    other => other.to_ascii_uppercase(),
  }
}

fn find_digit_index(c: char, alphabet: &[u8], is_standard: bool) -> Result<usize> {
  if is_standard {
    let target = normalize_char(c);
    alphabet
      .iter()
      .position(|&b| (b as char).to_ascii_uppercase() == target)
      .ok_or_else(|| Error::new(Status::InvalidArg, format!("Invalid character in string: {}", c)))
  } else {
    alphabet
      .iter()
      .position(|&b| (b as char) == c)
      .or_else(|| {
        alphabet
          .iter()
          .position(|&b| (b as char).to_ascii_uppercase() == c.to_ascii_uppercase())
      })
      .ok_or_else(|| Error::new(Status::InvalidArg, format!("Invalid character in string: {}", c)))
  }
}

#[napi(object)]
#[derive(Default)]
pub struct KsuidOptions {
  pub enc: Option<String>,
  pub encoding: Option<String>,
  pub alphabet: Option<String>,
  pub timestamp_size: Option<String>,
  pub bytes: Option<Uint8Array>,
  pub string: Option<String>,
  pub timestamp: Option<i64>,
}

#[napi]
pub struct Ksuid {
  bytes: [u8; 20],
  encoding: String,
  timestamp_size: String,
  custom_alphabet: Option<String>,
}

#[napi]
impl Ksuid {
  #[napi(constructor)]
  pub fn new(options: Option<KsuidOptions>) -> Result<Self> {
    let opts = options.unwrap_or_default();
    let encoding = opts
      .enc
      .or(opts.encoding)
      .unwrap_or_else(|| "base62".to_string());
    let ts_size = opts.timestamp_size.unwrap_or_else(|| "32bit".to_string());
    let custom_alphabet = opts.alphabet;

    let ksuid_bytes = if let Some(b) = opts.bytes {
      let slice = b.as_ref();
      if slice.len() != 20 {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Expected 20 bytes for KSUID, got {}", slice.len()),
        ));
      }
      let mut arr = [0u8; 20];
      arr.copy_from_slice(slice);
      arr
    } else if let Some(s) = opts.string {
      if encoding == "crockford" || encoding == "base32" {
        let alpha = custom_alphabet
          .clone()
          .unwrap_or_else(|| String::from_utf8_lossy(DEFAULT_CROCKFORD_ALPHABET).to_string());
        let decoded = decode_bytes_custom(s, alpha)?;
        let slice = decoded.as_ref();
        if slice.len() != 20 {
          return Err(Error::new(
            Status::InvalidArg,
            format!("Expected 20 bytes for KSUID, got {}", slice.len()),
          ));
        }
        let mut arr = [0u8; 20];
        arr.copy_from_slice(slice);
        arr
      } else {
        let ksuid = SvixKsuid::from_base62(&s).map_err(|e| {
          Error::new(Status::InvalidArg, format!("Invalid Base62 KSUID string: {}", e))
        })?;
        *ksuid.bytes()
      }
    } else {
      let mut arr = [0u8; 20];
      let random_ksuid = SvixKsuid::new(None, None);
      let rand_bytes = random_ksuid.bytes();

      match ts_size.as_str() {
        "48bit" => {
          let ms = opts.timestamp.map(|t| t as u64).unwrap_or_else(|| {
            std::time::SystemTime::now()
              .duration_since(std::time::UNIX_EPOCH)
              .unwrap_or_default()
              .as_millis() as u64
          });
          arr[0] = (ms >> 40) as u8;
          arr[1] = (ms >> 32) as u8;
          arr[2] = (ms >> 24) as u8;
          arr[3] = (ms >> 16) as u8;
          arr[4] = (ms >> 8) as u8;
          arr[5] = ms as u8;
          arr[6..20].copy_from_slice(&rand_bytes[6..20]);
        }
        "64bit" => {
          let ms = opts.timestamp.map(|t| t as u64).unwrap_or_else(|| {
            std::time::SystemTime::now()
              .duration_since(std::time::UNIX_EPOCH)
              .unwrap_or_default()
              .as_millis() as u64
          });
          arr[0..8].copy_from_slice(&ms.to_be_bytes());
          arr[8..20].copy_from_slice(&rand_bytes[8..20]);
        }
        _ => {
          if let Some(ts) = opts.timestamp {
            let secs = ts as u32;
            arr[0..4].copy_from_slice(&secs.to_be_bytes());
            arr[4..20].copy_from_slice(&rand_bytes[4..20]);
          } else {
            arr.copy_from_slice(rand_bytes);
          }
        }
      }
      arr
    };

    Ok(Self {
      bytes: ksuid_bytes,
      encoding,
      timestamp_size: ts_size,
      custom_alphabet,
    })
  }

  #[napi(factory)]
  pub fn now(options: Option<KsuidOptions>) -> Result<Self> {
    Self::new(options)
  }

  #[napi(factory)]
  pub fn parse(input: String, options: Option<KsuidOptions>) -> Result<Self> {
    let mut opts = options.unwrap_or_default();
    opts.string = Some(input);
    Self::new(Some(opts))
  }

  #[napi]
  pub fn to_string(&self) -> Result<String> {
    if self.encoding == "crockford" || self.encoding == "base32" {
      let alpha = self
        .custom_alphabet
        .clone()
        .unwrap_or_else(|| String::from_utf8_lossy(DEFAULT_CROCKFORD_ALPHABET).to_string());
      encode_bytes_custom(Uint8Array::from(self.bytes.to_vec()), alpha)
    } else {
      let ksuid = SvixKsuid::from_bytes(self.bytes);
      Ok(ksuid.to_string())
    }
  }

  #[napi]
  pub fn to_base62(&self) -> String {
    let ksuid = SvixKsuid::from_bytes(self.bytes);
    ksuid.to_string()
  }

  #[napi]
  pub fn to_crockford(&self, custom_alphabet: Option<String>) -> Result<String> {
    let alpha = custom_alphabet
      .or_else(|| self.custom_alphabet.clone())
      .unwrap_or_else(|| String::from_utf8_lossy(DEFAULT_CROCKFORD_ALPHABET).to_string());
    encode_bytes_custom(Uint8Array::from(self.bytes.to_vec()), alpha)
  }

  #[napi]
  pub fn bytes(&self) -> Uint8Array {
    Uint8Array::from(self.bytes.to_vec())
  }

  #[napi]
  pub fn timestamp_seconds(&self) -> i64 {
    match self.timestamp_size.as_str() {
      "48bit" | "64bit" => self.timestamp_millis() / 1000,
      _ => SvixKsuid::from_bytes(self.bytes).timestamp_seconds(),
    }
  }

  #[napi]
  pub fn timestamp_millis(&self) -> i64 {
    match self.timestamp_size.as_str() {
      "48bit" => {
        let ms = ((self.bytes[0] as u64) << 40)
          | ((self.bytes[1] as u64) << 32)
          | ((self.bytes[2] as u64) << 24)
          | ((self.bytes[3] as u64) << 16)
          | ((self.bytes[4] as u64) << 8)
          | (self.bytes[5] as u64);
        ms as i64
      }
      "64bit" => {
        let ms = u64::from_be_bytes([
          self.bytes[0],
          self.bytes[1],
          self.bytes[2],
          self.bytes[3],
          self.bytes[4],
          self.bytes[5],
          self.bytes[6],
          self.bytes[7],
        ]);
        ms as i64
      }
      _ => (SvixKsuid::from_bytes(self.bytes).timestamp_seconds() as i64) * 1000,
    }
  }
}

#[napi]
pub fn new_ksuid() -> String {
  SvixKsuid::new(None, None).to_string()
}

#[napi]
pub fn ksuid_from_base62(base62: String) -> Result<String> {
  let ksuid = SvixKsuid::from_base62(&base62)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid KSUID base62 string: {}", e)))?;
  Ok(ksuid.to_string())
}

#[napi]
pub fn ksuid_to_bytes(base62: String) -> Result<Uint8Array> {
  let ksuid = SvixKsuid::from_base62(&base62)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid KSUID base62 string: {}", e)))?;
  Ok(Uint8Array::from(ksuid.bytes().to_vec()))
}

#[napi]
pub fn ksuid_timestamp_seconds(base62: String) -> Result<i64> {
  let ksuid = SvixKsuid::from_base62(&base62)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid KSUID base62 string: {}", e)))?;
  Ok(ksuid.timestamp_seconds())
}

#[napi]
pub fn crockford_encode(n: f64, custom_alphabet: Option<String>) -> Result<String> {
  if n < 0.0 || n.is_nan() || n.is_infinite() {
    return Err(Error::new(Status::InvalidArg, "Value must be a non-negative number"));
  }
  let mut val = n as u64;
  let (alphabet, _) = get_alphabet(custom_alphabet)?;

  if val == 0 {
    return Ok((alphabet[0] as char).to_string());
  }

  let mut buf = Vec::new();
  while val > 0 {
    let digit = (val % 32) as usize;
    buf.push(alphabet[digit]);
    val /= 32;
  }
  buf.reverse();
  String::from_utf8(buf).map_err(|e| Error::new(Status::GenericFailure, e.to_string()))
}

#[napi]
pub fn crockford_decode(input: String, custom_alphabet: Option<String>) -> Result<f64> {
  if input.is_empty() {
    return Err(Error::new(Status::InvalidArg, "Input string cannot be empty"));
  }
  let (alphabet, is_standard) = get_alphabet(custom_alphabet)?;

  let mut val: u64 = 0;
  for c in input.chars() {
    let digit_idx = find_digit_index(c, &alphabet, is_standard)?;

    val = val
      .checked_mul(32)
      .and_then(|v| v.checked_add(digit_idx as u64))
      .ok_or_else(|| Error::new(Status::InvalidArg, "Number overflow in decode"))?;
  }

  Ok(val as f64)
}

#[napi]
pub fn encode_bytes_custom(bytes: Uint8Array, alphabet: String) -> Result<String> {
  let (alpha_bytes, _) = get_alphabet(Some(alphabet))?;
  let data = bytes.as_ref();

  if data.is_empty() {
    return Ok(String::new());
  }

  let mut result = String::new();
  let mut buffer: u32 = 0;
  let mut bits_left = 0;

  for &byte in data {
    buffer = (buffer << 8) | (byte as u32);
    bits_left += 8;
    while bits_left >= 5 {
      bits_left -= 5;
      let idx = ((buffer >> bits_left) & 0x1f) as usize;
      result.push(alpha_bytes[idx] as char);
    }
  }

  if bits_left > 0 {
    let idx = ((buffer << (5 - bits_left)) & 0x1f) as usize;
    result.push(alpha_bytes[idx] as char);
  }

  Ok(result)
}

#[napi]
pub fn decode_bytes_custom(input: String, alphabet: String) -> Result<Uint8Array> {
  let (alpha_bytes, is_standard) = get_alphabet(Some(alphabet))?;
  if input.is_empty() {
    return Ok(Uint8Array::from(vec![]));
  }

  let mut buffer: u32 = 0;
  let mut bits_left = 0;
  let mut out = Vec::new();

  for c in input.chars() {
    let idx = find_digit_index(c, &alpha_bytes, is_standard)?;

    buffer = (buffer << 5) | (idx as u32);
    bits_left += 5;

    if bits_left >= 8 {
      bits_left -= 8;
      let byte = ((buffer >> bits_left) & 0xff) as u8;
      out.push(byte);
    }
  }

  Ok(Uint8Array::from(out))
}

#[napi]
pub fn ksuid_to_crockford(base62: String, custom_alphabet: Option<String>) -> Result<String> {
  let ksuid = SvixKsuid::from_base62(&base62)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid KSUID base62 string: {}", e)))?;
  let bytes = Uint8Array::from(ksuid.bytes().to_vec());
  let alpha = custom_alphabet.unwrap_or_else(|| String::from_utf8_lossy(DEFAULT_CROCKFORD_ALPHABET).to_string());
  encode_bytes_custom(bytes, alpha)
}

#[napi]
pub fn ksuid_from_crockford(crockford: String, custom_alphabet: Option<String>) -> Result<String> {
  let alpha = custom_alphabet.unwrap_or_else(|| String::from_utf8_lossy(DEFAULT_CROCKFORD_ALPHABET).to_string());
  let bytes = decode_bytes_custom(crockford, alpha)?;
  let data = bytes.as_ref();
  if data.len() != 20 {
    return Err(Error::new(Status::InvalidArg, format!("Expected 20 bytes for KSUID, got {}", data.len())));
  }
  let mut array = [0u8; 20];
  array.copy_from_slice(data);
  let ksuid = SvixKsuid::from_bytes(array);
  Ok(ksuid.to_string())
}
