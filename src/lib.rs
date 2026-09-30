use napi::bindgen_prelude::*;
use napi_derive::napi;

pub const URL_ALPHABET: &str = "usemodule-aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ1234567890_-";

#[napi]
pub fn nanoid(size: Option<u32>) -> Result<String> {
  let size = size.unwrap_or(21) as usize;
  custom_alphabet(URL_ALPHABET.to_string(), Some(size as u32))
}

#[napi]
pub fn custom_alphabet(alphabet: String, size: Option<u32>) -> Result<String> {
  let size = size.unwrap_or(21) as usize;
  let chars: Vec<char> = alphabet.chars().collect();
  let len = chars.len();
  if len == 0 || len > 256 {
    return Err(Error::new(
      Status::InvalidArg,
      "Alphabet must contain from 1 to 256 symbols".to_string(),
    ));
  }
  Ok(nanoid::nanoid!(size, &chars))
}
