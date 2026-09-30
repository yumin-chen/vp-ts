use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub fn init() {
  btls::init();
}

#[napi]
pub fn version() -> String {
  btls::version::version().to_string()
}

#[napi]
pub fn encode_base64(data: Buffer) -> String {
  btls::base64::encode_block(&data)
}

#[napi]
pub fn decode_base64(str: String) -> Result<Buffer> {
  let bytes = btls::base64::decode_block(&str)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Failed to decode base64: {}", e)))?;
  Ok(Buffer::from(bytes))
}

#[napi]
pub fn sha256(data: Buffer) -> Buffer {
  let hash = btls::sha::sha256(&data);
  Buffer::from(&hash[..])
}

#[napi]
pub fn sha512(data: Buffer) -> Buffer {
  let hash = btls::sha::sha512(&data);
  Buffer::from(&hash[..])
}

#[napi]
pub fn sha1(data: Buffer) -> Buffer {
  let hash = btls::sha::sha1(&data);
  Buffer::from(&hash[..])
}

fn get_message_digest(alg: &str) -> Result<btls::hash::MessageDigest> {
  match alg.to_lowercase().replace('-', "").as_str() {
    "sha256" => Ok(btls::hash::MessageDigest::sha256()),
    "sha512" => Ok(btls::hash::MessageDigest::sha512()),
    "sha1" => Ok(btls::hash::MessageDigest::sha1()),
    "sha384" => Ok(btls::hash::MessageDigest::sha384()),
    "md5" => Ok(btls::hash::MessageDigest::md5()),
    _ => Err(Error::new(
      Status::InvalidArg,
      format!("Unsupported hash algorithm: {}", alg),
    )),
  }
}

#[napi]
pub fn digest(algorithm: String, data: Buffer) -> Result<Buffer> {
  let md = get_message_digest(&algorithm)?;
  let hash = btls::hash::hash(md, &data)
    .map_err(|e| Error::new(Status::GenericFailure, format!("Digest failed: {}", e)))?;
  Ok(Buffer::from(&hash[..]))
}

#[napi]
pub fn hmac(algorithm: String, key: Buffer, data: Buffer) -> Result<Buffer> {
  match algorithm.to_lowercase().replace('-', "").as_str() {
    "sha256" => {
      let res = btls::hash::hmac_sha256(&key, &data)
        .map_err(|e| Error::new(Status::GenericFailure, format!("HMAC failed: {}", e)))?;
      Ok(Buffer::from(&res[..]))
    }
    "sha512" => {
      let res = btls::hash::hmac_sha512(&key, &data)
        .map_err(|e| Error::new(Status::GenericFailure, format!("HMAC failed: {}", e)))?;
      Ok(Buffer::from(&res[..]))
    }
    "sha1" => {
      let res = btls::hash::hmac_sha1(&key, &data)
        .map_err(|e| Error::new(Status::GenericFailure, format!("HMAC failed: {}", e)))?;
      Ok(Buffer::from(&res[..]))
    }
    _ => Err(Error::new(
      Status::InvalidArg,
      format!("Unsupported HMAC algorithm: {}", algorithm),
    )),
  }
}

fn get_cipher(cipher_name: &str) -> Result<btls::symm::Cipher> {
  match cipher_name.to_lowercase().replace('-', "").as_str() {
    "aes128cbc" => Ok(btls::symm::Cipher::aes_128_cbc()),
    "aes128ecb" => Ok(btls::symm::Cipher::aes_128_ecb()),
    "aes256cbc" => Ok(btls::symm::Cipher::aes_256_cbc()),
    "aes256ecb" => Ok(btls::symm::Cipher::aes_256_ecb()),
    "aes128gcm" => Ok(btls::symm::Cipher::aes_128_gcm()),
    "aes256gcm" => Ok(btls::symm::Cipher::aes_256_gcm()),
    _ => Err(Error::new(
      Status::InvalidArg,
      format!("Unsupported cipher: {}", cipher_name),
    )),
  }
}

#[napi]
pub fn encrypt(
  cipher_name: String,
  key: Buffer,
  iv: Option<Buffer>,
  data: Buffer,
) -> Result<Buffer> {
  let cipher = get_cipher(&cipher_name)?;
  let iv_slice = iv.as_deref();
  let encrypted = btls::symm::encrypt(cipher, &key, iv_slice, &data)
    .map_err(|e| Error::new(Status::GenericFailure, format!("Encryption failed: {}", e)))?;
  Ok(Buffer::from(encrypted))
}

#[napi]
pub fn decrypt(
  cipher_name: String,
  key: Buffer,
  iv: Option<Buffer>,
  data: Buffer,
) -> Result<Buffer> {
  let cipher = get_cipher(&cipher_name)?;
  let iv_slice = iv.as_deref();
  let decrypted = btls::symm::decrypt(cipher, &key, iv_slice, &data)
    .map_err(|e| Error::new(Status::GenericFailure, format!("Decryption failed: {}", e)))?;
  Ok(Buffer::from(decrypted))
}

#[napi]
pub fn random_bytes(length: u32) -> Result<Buffer> {
  let mut buf = vec![0u8; length as usize];
  btls::rand::rand_bytes(&mut buf)
    .map_err(|e| Error::new(Status::GenericFailure, format!("Random generation failed: {}", e)))?;
  Ok(Buffer::from(buf))
}

#[napi]
pub struct BoringSshCodec {}

#[napi]
impl BoringSshCodec {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self {}
  }

  #[napi]
  pub fn encode(&self, data: Buffer) -> String {
    encode_base64(data)
  }

  #[napi]
  pub fn decode(&self, str: String) -> Result<Buffer> {
    decode_base64(str)
  }

  #[napi]
  pub fn hash(&self, algorithm: Option<String>, data: Buffer) -> Result<Buffer> {
    let alg = algorithm.unwrap_or_else(|| "sha256".to_string());
    digest(alg, data)
  }

  #[napi]
  pub fn hmac(&self, algorithm: Option<String>, key: Buffer, data: Buffer) -> Result<Buffer> {
    let alg = algorithm.unwrap_or_else(|| "sha256".to_string());
    hmac(alg, key, data)
  }

  #[napi]
  pub fn encrypt(
    &self,
    cipher: String,
    key: Buffer,
    iv: Option<Buffer>,
    data: Buffer,
  ) -> Result<Buffer> {
    encrypt(cipher, key, iv, data)
  }

  #[napi]
  pub fn decrypt(
    &self,
    cipher: String,
    key: Buffer,
    iv: Option<Buffer>,
    data: Buffer,
  ) -> Result<Buffer> {
    decrypt(cipher, key, iv, data)
  }

  #[napi]
  pub fn random_bytes(&self, length: u32) -> Result<Buffer> {
    random_bytes(length)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_base64() {
    let text = b"hello btls world";
    let encoded = encode_base64(Buffer::from(text.as_slice()));
    let decoded = decode_base64(encoded).unwrap();
    assert_eq!(&decoded[..], text);
  }

  #[test]
  fn test_sha256_and_digest() {
    let text = b"test message";
    let hash1 = sha256(Buffer::from(text.as_slice()));
    let hash2 = digest("sha256".to_string(), Buffer::from(text.as_slice())).unwrap();
    assert_eq!(&hash1[..], &hash2[..]);
  }

  #[test]
  fn test_encrypt_decrypt() {
    let key = vec![0u8; 16];
    let iv = vec![0u8; 16];
    let data = b"hello encryption world";

    let encrypted = encrypt(
      "aes-128-cbc".to_string(),
      Buffer::from(key.clone()),
      Some(Buffer::from(iv.clone())),
      Buffer::from(data.as_slice()),
    )
    .unwrap();

    let decrypted = decrypt(
      "aes-128-cbc".to_string(),
      Buffer::from(key),
      Some(Buffer::from(iv)),
      encrypted,
    )
    .unwrap();

    assert_eq!(&decrypted[..], data);
  }

  #[test]
  fn test_random_bytes() {
    let bytes = random_bytes(32).unwrap();
    assert_eq!(bytes.len(), 32);
  }
}
