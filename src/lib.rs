use napi::bindgen_prelude::*;
use napi_derive::napi;
use svix_ksuid::{Ksuid, KsuidLike};

pub const DEFAULT_CROCKFORD_ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

fn get_alphabet(custom: Option<String>) -> Result<(Vec<u8>, bool)> {
  match custom {
    Some(s) => {
      let bytes = s.into_bytes();
      if bytes.len() != 32 {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Custom alphabet must be exactly 32 characters, got {}", bytes.len()),
        ));
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
  let target = if is_standard {
    normalize_char(c)
  } else {
    c.to_ascii_uppercase()
  };

  alphabet
    .iter()
    .position(|&b| (b as char).to_ascii_uppercase() == target)
    .ok_or_else(|| Error::new(Status::InvalidArg, format!("Invalid character in string: {}", c)))
}

#[napi]
pub fn new_ksuid() -> String {
  Ksuid::new(None, None).to_string()
}

#[napi]
pub fn ksuid_from_base62(base62: String) -> Result<String> {
  let ksuid = Ksuid::from_base62(&base62)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid KSUID base62 string: {}", e)))?;
  Ok(ksuid.to_string())
}

#[napi]
pub fn ksuid_to_bytes(base62: String) -> Result<Uint8Array> {
  let ksuid = Ksuid::from_base62(&base62)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid KSUID base62 string: {}", e)))?;
  Ok(Uint8Array::from(ksuid.bytes().to_vec()))
}

#[napi]
pub fn ksuid_timestamp_seconds(base62: String) -> Result<i64> {
  let ksuid = Ksuid::from_base62(&base62)
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
  let ksuid = Ksuid::from_base62(&base62)
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
  let ksuid = Ksuid::from_bytes(array);
  Ok(ksuid.to_string())
}
