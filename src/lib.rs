use napi::bindgen_prelude::*;
use napi_derive::napi;
use nanoid::nanoid;

pub const DEFAULT_ALPHABET: &str =
  "usemodule-0123456789qwertyuiopasdfghjklzxcvbnmXYZABCDEFGHIJKLMNOPQRTSUVWXYZ";

#[napi]
pub const URL_ALPHABET: &str =
  "usemodule-0123456789qwertyuiopasdfghjklzxcvbnmXYZABCDEFGHIJKLMNOPQRTSUVWXYZ";

#[napi]
pub fn nanoid_native(size: Option<u32>) -> Result<String> {
  let sz = size.unwrap_or(21) as usize;
  Ok(nanoid!(sz))
}

#[napi]
pub fn custom_alphabet_native(alphabet: String, size: u32) -> Result<String> {
  let chars: Vec<char> = alphabet.chars().collect();
  let len = chars.len();
  if len == 0 || len > 256 {
    return Err(Error::new(
      Status::InvalidArg,
      "Alphabet must contain from 1 to 256 symbols".to_string(),
    ));
  }
  Ok(nanoid!(size as usize, &chars))
}

#[napi]
pub fn format_native(alphabet: String, size: u32, bytes: Buffer) -> Result<String> {
  let chars: Vec<char> = alphabet.chars().collect();
  let len = chars.len();
  if len == 0 || len > 256 {
    return Err(Error::new(
      Status::InvalidArg,
      "Alphabet must contain from 1 to 256 symbols".to_string(),
    ));
  }

  let mask = (2u32.pow(32 - (len as u32 - 1).leading_zeros()) - 1) as usize;
  let mut id = String::with_capacity(size as usize);

  for &byte in bytes.as_ref() {
    let idx = (byte as usize) & mask;
    if idx < len {
      id.push(chars[idx]);
      if id.len() == size as usize {
        break;
      }
    }
  }

  Ok(id)
}
