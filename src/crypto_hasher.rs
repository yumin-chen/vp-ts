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
pub fn get_hashes() -> Vec<String> {
  vec![
    "md5".to_string(),
    "sha1".to_string(),
    "sha256".to_string(),
    "sha384".to_string(),
    "sha512".to_string(),
  ]
}
