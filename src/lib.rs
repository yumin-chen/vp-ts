use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub fn add(left: i32, right: i32) -> i32 {
  left + right
}

#[napi]
pub fn init() {
  btls::init();
}

#[napi]
pub fn version() -> String {
  btls::version::version().to_string()
}

#[napi]
pub fn base64_encode(data: Buffer) -> String {
  btls::base64::encode_block(data.as_ref())
}

#[napi]
pub fn base64_decode(encoded: String) -> Result<Buffer> {
  btls::base64::decode_block(&encoded)
    .map(Buffer::from)
    .map_err(|e| Error::from_reason(format!("Base64 decode error: {e}")))
}

#[napi]
pub fn hash(algorithm: String, data: Buffer) -> Result<Buffer> {
  let digest = match algorithm.to_lowercase().as_str() {
    "sha1" => btls::hash::MessageDigest::sha1(),
    "sha224" => btls::hash::MessageDigest::sha224(),
    "sha256" => btls::hash::MessageDigest::sha256(),
    "sha384" => btls::hash::MessageDigest::sha384(),
    "sha512" => btls::hash::MessageDigest::sha512(),
    "md5" => btls::hash::MessageDigest::md5(),
    _ => return Err(Error::from_reason(format!("Unsupported hash algorithm: {algorithm}"))),
  };

  btls::hash::hash(digest, data.as_ref())
    .map(|res| Buffer::from(res.as_ref()))
    .map_err(|e| Error::from_reason(format!("Hash error: {e}")))
}

fn parse_cipher(cipher_name: &str) -> Result<btls::symm::Cipher> {
  match cipher_name.to_lowercase().replace('_', "-").as_str() {
    "aes-128-cbc" => Ok(btls::symm::Cipher::aes_128_cbc()),
    "aes-128-ecb" => Ok(btls::symm::Cipher::aes_128_ecb()),
    "aes-128-ctr" => Ok(btls::symm::Cipher::aes_128_ctr()),
    "aes-256-cbc" => Ok(btls::symm::Cipher::aes_256_cbc()),
    "aes-256-ecb" => Ok(btls::symm::Cipher::aes_256_ecb()),
    "aes-256-ctr" => Ok(btls::symm::Cipher::aes_256_ctr()),
    "aes-128-gcm" => Ok(btls::symm::Cipher::aes_128_gcm()),
    "aes-256-gcm" => Ok(btls::symm::Cipher::aes_256_gcm()),
    _ => Err(Error::from_reason(format!("Unsupported cipher algorithm: {cipher_name}"))),
  }
}

#[napi]
pub fn encrypt(
  cipher_name: String,
  key: Buffer,
  iv: Option<Buffer>,
  data: Buffer,
) -> Result<Buffer> {
  let cipher = parse_cipher(&cipher_name)?;
  let iv_ref = iv.as_ref().map(|b: &Buffer| b.as_ref());
  btls::symm::encrypt(cipher, key.as_ref(), iv_ref, data.as_ref())
    .map(Buffer::from)
    .map_err(|e| Error::from_reason(format!("Encrypt error: {e}")))
}

#[napi]
pub fn decrypt(
  cipher_name: String,
  key: Buffer,
  iv: Option<Buffer>,
  data: Buffer,
) -> Result<Buffer> {
  let cipher = parse_cipher(&cipher_name)?;
  let iv_ref = iv.as_ref().map(|b: &Buffer| b.as_ref());
  btls::symm::decrypt(cipher, key.as_ref(), iv_ref, data.as_ref())
    .map(Buffer::from)
    .map_err(|e| Error::from_reason(format!("Decrypt error: {e}")))
}

#[napi]
pub struct BoringCodec {
  cipher_name: String,
  key: Vec<u8>,
  iv: Option<Vec<u8>>,
}

#[napi]
impl BoringCodec {
  #[napi(constructor)]
  pub fn new(cipher_name: String, key: Buffer, iv: Option<Buffer>) -> Self {
    Self {
      cipher_name,
      key: key.as_ref().to_vec(),
      iv: iv.map(|b: Buffer| b.as_ref().to_vec()),
    }
  }

  #[napi]
  pub fn encode(&self, data: Buffer) -> Result<Buffer> {
    let cipher = parse_cipher(&self.cipher_name)?;
    let iv_ref = self.iv.as_deref();
    btls::symm::encrypt(cipher, &self.key, iv_ref, data.as_ref())
      .map(Buffer::from)
      .map_err(|e| Error::from_reason(format!("Codec encode error: {e}")))
  }

  #[napi]
  pub fn decode(&self, data: Buffer) -> Result<Buffer> {
    let cipher = parse_cipher(&self.cipher_name)?;
    let iv_ref = self.iv.as_deref();
    btls::symm::decrypt(cipher, &self.key, iv_ref, data.as_ref())
      .map(Buffer::from)
      .map_err(|e| Error::from_reason(format!("Codec decode error: {e}")))
  }
}
