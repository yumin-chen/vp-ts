use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::hmac::{Algorithm, Context, Key, HMAC_SHA1_FOR_LEGACY_USE_ONLY, HMAC_SHA256, HMAC_SHA384, HMAC_SHA512};

pub fn to_hex(bytes: &[u8]) -> String {
  let mut s = String::with_capacity(bytes.len() * 2);
  for &b in bytes {
    s.push_str(&format!("{:02x}", b));
  }
  s
}

pub fn from_hex(s: &str) -> std::result::Result<Vec<u8>, String> {
  let s = s.trim();
  if s.len() % 2 != 0 {
    return Err("Invalid hex length".to_string());
  }
  (0..s.len())
    .step_by(2)
    .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string()))
    .collect()
}

const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn to_base64(bytes: &[u8]) -> String {
  let mut res = String::new();
  let mut i = 0;
  while i < bytes.len() {
    let b0 = bytes[i] as u32;
    let b1 = if i + 1 < bytes.len() { bytes[i + 1] as u32 } else { 0 };
    let b2 = if i + 2 < bytes.len() { bytes[i + 2] as u32 } else { 0 };
    let triple = (b0 << 16) | (b1 << 8) | b2;

    res.push(BASE64_CHARS[((triple >> 18) & 63) as usize] as char);
    res.push(BASE64_CHARS[((triple >> 12) & 63) as usize] as char);
    if i + 1 < bytes.len() {
      res.push(BASE64_CHARS[((triple >> 6) & 63) as usize] as char);
    } else {
      res.push('=');
    }
    if i + 2 < bytes.len() {
      res.push(BASE64_CHARS[(triple & 63) as usize] as char);
    } else {
      res.push('=');
    }
    i += 3;
  }
  res
}

pub fn from_base64(s: &str) -> std::result::Result<Vec<u8>, String> {
  let s = s.trim().trim_end_matches('=');
  let mut out = Vec::new();
  let mut val = 0u32;
  let mut valb = -8i32;

  for c in s.bytes() {
    let d = match c {
      b'A'..=b'Z' => c - b'A',
      b'a'..=b'z' => c - b'a' + 26,
      b'0'..=b'9' => c - b'0' + 52,
      b'+' => 62,
      b'/' => 63,
      _ => continue,
    };
    val = (val << 6) | (d as u32);
    valb += 6;
    if valb >= 0 {
      out.push(((val >> valb) & 0xFF) as u8);
      valb -= 8;
    }
  }
  Ok(out)
}

fn get_hmac_algorithm(algo: &str) -> napi::Result<Algorithm> {
  match algo.to_lowercase().replace("-", "").as_str() {
    "sha1" => Ok(HMAC_SHA1_FOR_LEGACY_USE_ONLY),
    "sha256" => Ok(HMAC_SHA256),
    "sha384" => Ok(HMAC_SHA384),
    "sha512" => Ok(HMAC_SHA512),
    _ => Err(napi::Error::from_reason(format!("Unsupported HMAC algorithm: {}", algo))),
  }
}

pub fn decode_input(data: &Either<String, Buffer>, encoding: Option<&str>) -> Vec<u8> {
  match data {
    Either::A(s) => match encoding {
      Some("hex") => from_hex(s).unwrap_or_else(|_| s.as_bytes().to_vec()),
      Some("base64") => from_base64(s).unwrap_or_else(|_| s.as_bytes().to_vec()),
      Some("latin1") | Some("binary") => s.chars().map(|c| c as u8).collect(),
      _ => s.as_bytes().to_vec(),
    },
    Either::B(buf) => buf.to_vec(),
  }
}

pub fn encode_output(bytes: &[u8], encoding: Option<&str>) -> Either<String, Buffer> {
  match encoding {
    Some("hex") => Either::A(to_hex(bytes)),
    Some("base64") => Either::A(to_base64(bytes)),
    Some("latin1") | Some("binary") => Either::A(bytes.iter().map(|&b| b as char).collect()),
    Some("utf8") | Some("utf-8") => Either::A(String::from_utf8_lossy(bytes).to_string()),
    _ => Either::B(Buffer::from(bytes)),
  }
}

#[napi]
pub struct Hmac {
  ctx: Option<Context>,
}

#[napi]
impl Hmac {
  #[napi(constructor)]
  pub fn new(algorithm: String, key: Either<String, Buffer>) -> napi::Result<Self> {
    let algo = get_hmac_algorithm(&algorithm)?;
    let key_bytes = decode_input(&key, None);
    let pkey = Key::new(algo, &key_bytes);
    let ctx = Context::with_key(&pkey);
    Ok(Hmac { ctx: Some(ctx) })
  }

  #[napi]
  pub fn update(&mut self, data: Either<String, Buffer>, encoding: Option<String>) -> napi::Result<()> {
    if let Some(ctx) = &mut self.ctx {
      let bytes = decode_input(&data, encoding.as_deref());
      ctx.update(&bytes);
      Ok(())
    } else {
      Err(napi::Error::from_reason("Hmac digest already called"))
    }
  }

  #[napi]
  pub fn digest(&mut self, encoding: Option<String>) -> napi::Result<Either<String, Buffer>> {
    if let Some(ctx) = self.ctx.take() {
      let tag = ctx.sign();
      Ok(encode_output(tag.as_ref(), encoding.as_deref()))
    } else {
      Err(napi::Error::from_reason("Hmac digest already called"))
    }
  }
}

#[napi]
pub fn create_hmac(algorithm: String, key: Either<String, Buffer>) -> napi::Result<Hmac> {
  Hmac::new(algorithm, key)
}
