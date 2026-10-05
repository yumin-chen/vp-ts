use base64ct::{Base64, Encoding};
use digest::Digest;
use md5::Md5;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use sha1::Sha1;
use sha2::{Sha256, Sha384, Sha512};

#[napi]
pub struct Hash {
  algorithm: String,
  data: Vec<u8>,
}

#[napi]
impl Hash {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> Self {
    Hash {
      algorithm,
      data: Vec::new(),
    }
  }

  #[napi]
  pub fn update(&mut self, data: Either<Buffer, String>) -> Result<()> {
    match data {
      Either::A(b) => self.data.extend_from_slice(b.as_ref()),
      Either::B(s) => self.data.extend_from_slice(s.as_bytes()),
    }
    Ok(())
  }

  #[napi]
  pub fn digest(&mut self, encoding: Option<String>) -> Result<Either<Buffer, String>> {
    let alg = self.algorithm.to_lowercase().replace('-', "");
    let bytes = match alg.as_str() {
      "sha256" => Sha256::digest(&self.data).to_vec(),
      "sha384" => Sha384::digest(&self.data).to_vec(),
      "sha512" => Sha512::digest(&self.data).to_vec(),
      "sha1" => Sha1::digest(&self.data).to_vec(),
      "md5" => Md5::digest(&self.data).to_vec(),
      other => return Err(Error::from_reason(format!("Unsupported hash algorithm: {other}"))),
    };

    match encoding.as_deref() {
      Some("hex") => Ok(Either::B(hex::encode(&bytes))),
      Some("base64") => Ok(Either::B(Base64::encode_string(&bytes))),
      _ => Ok(Either::A(Buffer::from(bytes))),
    }
  }
}

#[napi(js_name = "createHash")]
pub fn create_hash(algorithm: String) -> Hash {
  Hash::new(algorithm)
}

#[napi(js_name = "hash")]
pub fn hash_one_shot(
  algorithm: String,
  data: Either<Buffer, String>,
  encoding: Option<String>,
) -> Result<Either<Buffer, String>> {
  let mut h = Hash::new(algorithm);
  h.update(data)?;
  h.digest(encoding)
}

#[napi(js_name = "getHashes")]
pub fn get_hashes() -> Result<Vec<String>> {
  Ok(vec![
    "md5".to_string(),
    "sha1".to_string(),
    "sha256".to_string(),
    "sha384".to_string(),
    "sha512".to_string(),
  ])
}

#[napi(js_name = "getCiphers")]
pub fn get_ciphers() -> Result<Vec<String>> {
  Ok(vec![
    "aes-128-cbc".to_string(),
    "aes-128-ccm".to_string(),
    "aes-128-gcm".to_string(),
    "aes-192-cbc".to_string(),
    "aes-256-cbc".to_string(),
    "aes-256-gcm".to_string(),
    "chacha20-poly1305".to_string(),
  ])
}

#[napi(object)]
pub struct CipherInfo {
  pub name: String,
  pub block_size: Option<u32>,
  pub iv_length: Option<u32>,
  pub key_length: u32,
  pub mode: String,
}

#[napi(js_name = "getCipherInfo")]
pub fn get_cipher_info(name: String) -> Result<CipherInfo> {
  Ok(CipherInfo {
    name,
    block_size: Some(16),
    iv_length: Some(12),
    key_length: 32,
    mode: "gcm".to_string(),
  })
}

#[napi(js_name = "getCurves")]
pub fn get_curves() -> Result<Vec<String>> {
  Ok(vec![
    "prime256v1".to_string(),
    "secp256r1".to_string(),
    "secp384r1".to_string(),
    "secp521r1".to_string(),
    "x25519".to_string(),
    "ed25519".to_string(),
  ])
}

#[napi(js_name = "getMacs")]
pub fn get_macs() -> Result<Vec<String>> {
  Ok(vec!["cmac".to_string(), "gmac".to_string(), "hmac".to_string()])
}

#[napi(js_name = "getFips")]
pub fn get_fips() -> Result<u32> {
  Ok(0)
}

#[napi(js_name = "setFips")]
pub fn set_fips(_val: bool) -> Result<()> {
  Ok(())
}

#[napi(js_name = "setEngine")]
pub fn set_engine(_engine: String, _flags: Option<u32>) -> Result<()> {
  Ok(())
}

#[napi(object)]
pub struct SecureHeapInfo {
  pub total: u32,
  pub min: u32,
  pub used: u32,
  pub utilization: f64,
}

#[napi(js_name = "secureHeapUsed")]
pub fn secure_heap_used() -> Result<SecureHeapInfo> {
  Ok(SecureHeapInfo {
    total: 0,
    min: 0,
    used: 0,
    utilization: 0.0,
  })
}
