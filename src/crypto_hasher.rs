use napi::bindgen_prelude::*;
use napi_derive::napi;
use sha2::{Digest, Sha256, Sha384, Sha512, Sha224, Sha512_256};
use sha1::Sha1;

enum AnyHasher {
  Sha1(Sha1),
  Sha224(Sha224),
  Sha256(Sha256),
  Sha384(Sha384),
  Sha512(Sha512),
  Sha512_256(Sha512_256),
}

#[napi]
pub struct Hash {
  hasher: AnyHasher,
}

#[napi]
impl Hash {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> Result<Self> {
    let algo_clean = algorithm.to_lowercase().replace('-', "");
    let hasher = match algo_clean.as_str() {
      "sha1" => AnyHasher::Sha1(Sha1::new()),
      "sha224" => AnyHasher::Sha224(Sha224::new()),
      "sha256" => AnyHasher::Sha256(Sha256::new()),
      "sha384" => AnyHasher::Sha384(Sha384::new()),
      "sha512" => AnyHasher::Sha512(Sha512::new()),
      "sha512256" => AnyHasher::Sha512_256(Sha512_256::new()),
      _ => return Err(Error::new(Status::InvalidArg, format!("Unknown message digest: {}", algorithm))),
    };
    Ok(Self { hasher })
  }

  #[napi]
  pub fn update(&mut self, data: Buffer) -> Result<()> {
    match &mut self.hasher {
      AnyHasher::Sha1(h) => h.update(&data),
      AnyHasher::Sha224(h) => h.update(&data),
      AnyHasher::Sha256(h) => h.update(&data),
      AnyHasher::Sha384(h) => h.update(&data),
      AnyHasher::Sha512(h) => h.update(&data),
      AnyHasher::Sha512_256(h) => h.update(&data),
    }
    Ok(())
  }

  #[napi]
  pub fn digest(&mut self, encoding: Option<String>) -> Result<Either<Buffer, String>> {
    let bytes = match std::mem::replace(&mut self.hasher, AnyHasher::Sha256(Sha256::new())) {
      AnyHasher::Sha1(h) => h.finalize().to_vec(),
      AnyHasher::Sha224(h) => h.finalize().to_vec(),
      AnyHasher::Sha256(h) => h.finalize().to_vec(),
      AnyHasher::Sha384(h) => h.finalize().to_vec(),
      AnyHasher::Sha512(h) => h.finalize().to_vec(),
      AnyHasher::Sha512_256(h) => h.finalize().to_vec(),
    };

    match encoding.as_deref() {
      Some("hex") => Ok(Either::B(hex::encode(bytes))),
      Some("base64") => {
        use base64ct::{Base64, Encoding};
        Ok(Either::B(Base64::encode_string(&bytes)))
      }
      _ => Ok(Either::A(Buffer::from(bytes))),
    }
  }
}

#[napi]
pub fn create_hash(algorithm: String) -> Result<Hash> {
  Hash::new(algorithm)
}

#[napi]
pub fn hash(algorithm: String, data: Buffer, output_encoding: Option<String>) -> Result<Either<Buffer, String>> {
  let mut hasher = Hash::new(algorithm)?;
  hasher.update(data)?;
  hasher.digest(output_encoding)
}

#[napi]
pub fn get_hashes() -> Vec<String> {
  vec![
    "sha1".to_string(),
    "sha224".to_string(),
    "sha256".to_string(),
    "sha384".to_string(),
    "sha512".to_string(),
    "sha512-256".to_string(),
  ]
}
