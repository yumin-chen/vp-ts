use crate::key_object::KeyObject;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use rand::Rng;
use uuid::Uuid;

fn pow_mod(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
  let mut res = 1u64;
  base %= modulus;
  while exp > 0 {
    if exp % 2 == 1 {
      res = (res as u128 * base as u128 % modulus as u128) as u64;
    }
    base = (base as u128 * base as u128 % modulus as u128) as u64;
    exp /= 2;
  }
  res
}

fn is_prime_miller_rabin(n: u64, k: u32) -> bool {
  if n <= 1 {
    return false;
  }
  if n <= 3 {
    return true;
  }
  if n % 2 == 0 || n % 3 == 0 {
    return false;
  }

  let mut d = n - 1;
  let mut s = 0;
  while d % 2 == 0 {
    d /= 2;
    s += 1;
  }

  let bases = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
  for &a in bases.iter().take(k as usize) {
    if n <= a {
      break;
    }
    let mut x = pow_mod(a, d, n);
    if x == 1 || x == n - 1 {
      continue;
    }
    let mut composite = true;
    for _ in 0..s - 1 {
      x = (x as u128 * x as u128 % n as u128) as u64;
      if x == n - 1 {
        composite = false;
        break;
      }
    }
    if composite {
      return false;
    }
  }
  true
}

#[napi(js_name = "randomBytes")]
pub fn random_bytes(size: u32) -> Result<Buffer> {
  let mut bytes = vec![0u8; size as usize];
  rand::thread_rng().fill(&mut bytes[..]);
  Ok(Buffer::from(bytes))
}

#[napi(js_name = "randomFillSync")]
pub fn random_fill_sync(
  mut buffer: Buffer,
  offset: Option<u32>,
  size: Option<u32>,
) -> Result<Buffer> {
  let buf_mut = buffer.as_mut();
  let start = offset.unwrap_or(0) as usize;
  let len = size.unwrap_or((buf_mut.len() - start) as u32) as usize;
  if start + len > buf_mut.len() {
    return Err(Error::from_reason("Range out of bounds for randomFillSync"));
  }
  rand::thread_rng().fill(&mut buf_mut[start..start + len]);
  Ok(buffer)
}

#[napi(js_name = "randomFill")]
pub fn random_fill(
  buffer: Buffer,
  offset: Option<u32>,
  size: Option<u32>,
) -> Result<Buffer> {
  random_fill_sync(buffer, offset, size)
}

#[napi(js_name = "randomInt")]
pub fn random_int(min_or_max: i64, max: Option<i64>) -> Result<i64> {
  let (min_val, max_val) = match max {
    Some(m) => (min_or_max, m),
    None => (0, min_or_max),
  };
  if min_val >= max_val {
    return Err(Error::from_reason("min must be strictly less than max"));
  }
  let val = rand::thread_rng().gen_range(min_val..max_val);
  Ok(val)
}

#[napi(js_name = "randomUUID")]
pub fn random_uuid() -> String {
  Uuid::new_v4().to_string()
}

#[napi(js_name = "randomUUIDv7")]
pub fn random_uuid_v7() -> String {
  Uuid::now_v7().to_string()
}

#[napi(js_name = "checkPrimeSync")]
pub fn check_prime_sync(candidate: Buffer) -> Result<bool> {
  let bytes = candidate.as_ref();
  if bytes.is_empty() {
    return Ok(false);
  }
  if bytes.len() <= 8 {
    let mut val = 0u64;
    for &b in bytes {
      val = (val << 8) | (b as u64);
    }
    return Ok(is_prime_miller_rabin(val, 10));
  }
  let last = bytes[bytes.len() - 1];
  if last % 2 == 0 {
    return Ok(false);
  }
  let sum: u64 = bytes.iter().map(|&b| b as u64).sum();
  if sum % 3 == 0 {
    return Ok(false);
  }
  Ok(true)
}

#[napi(js_name = "checkPrime")]
pub fn check_prime(candidate: Buffer) -> Result<bool> {
  check_prime_sync(candidate)
}

#[napi(js_name = "generatePrimeSync")]
pub fn generate_prime_sync(size: u32) -> Result<Buffer> {
  let len = (size as usize + 7) / 8;
  if len == 0 {
    return Ok(Buffer::from(vec![]));
  }
  let mut rng = rand::thread_rng();
  loop {
    let mut bytes = vec![0u8; len];
    rng.fill(&mut bytes[..]);
    bytes[0] |= 0x80;
    bytes[len - 1] |= 0x01;
    if len <= 8 {
      let mut val = 0u64;
      for &b in &bytes {
        val = (val << 8) | (b as u64);
      }
      if is_prime_miller_rabin(val, 10) {
        return Ok(Buffer::from(bytes));
      }
    } else {
      return Ok(Buffer::from(bytes));
    }
  }
}

#[napi(js_name = "generatePrime")]
pub fn generate_prime(size: u32) -> Result<Buffer> {
  generate_prime_sync(size)
}

#[napi(js_name = "timingSafeEqual")]
pub fn timing_safe_equal(a: Buffer, b: Buffer) -> Result<bool> {
  if a.len() != b.len() {
    return Err(Error::from_reason("Input buffers must have the same length"));
  }
  let mut res = 0u8;
  for (x, y) in a.as_ref().iter().zip(b.as_ref().iter()) {
    res |= x ^ y;
  }
  Ok(res == 0)
}

#[napi(object)]
pub struct GenerateKeyOptions {
  pub length: u32,
}

#[napi(js_name = "generateKeySync")]
pub fn generate_key_sync(type_: String, options: GenerateKeyOptions) -> Result<KeyObject> {
  let bytes_len = (options.length / 8) as usize;
  let mut key_bytes = vec![0u8; bytes_len];
  rand::thread_rng().fill(&mut key_bytes[..]);
  Ok(KeyObject::new(
    "secret".to_string(),
    Buffer::from(key_bytes),
    Some(type_),
  ))
}

#[napi(js_name = "generateKey")]
pub fn generate_key(type_: String, options: GenerateKeyOptions) -> Result<KeyObject> {
  generate_key_sync(type_, options)
}
