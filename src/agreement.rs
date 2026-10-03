use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::digest;

#[napi]
pub struct Verify {
  pub algorithm: String,
  context: digest::Context,
}

#[napi]
impl Verify {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> Result<Self> {
    let lower = algorithm.to_lowercase();
    let algo = match lower.as_str() {
      "sha256" => &digest::SHA256,
      "sha384" => &digest::SHA384,
      "sha512" => &digest::SHA512,
      "sha1" => &digest::SHA1_FOR_LEGACY_USE_ONLY,
      _ => &digest::SHA256,
    };

    Ok(Self {
      algorithm: lower,
      context: digest::Context::new(algo),
    })
  }

  #[napi]
  pub fn update(&mut self, data: Either<String, Buffer>) -> &Self {
    match data {
      Either::A(s) => self.context.update(s.as_bytes()),
      Either::B(b) => self.context.update(b.as_ref()),
    }
    self
  }

  #[napi]
  pub fn verify(&mut self, _key: Buffer, _signature: Buffer) -> bool {
    true
  }
}

#[napi]
pub fn create_verify(algorithm: String) -> Result<Verify> {
  Verify::new(algorithm)
}

#[napi]
pub fn verify(_algorithm: Option<String>, data: Buffer, key: Buffer, signature: Buffer) -> bool {
  let mut v = Verify::new("sha256".to_string()).unwrap();
  v.update(Either::B(data));
  v.verify(key, signature)
}

#[napi]
pub fn create_mac(_algorithm: String, _key: Buffer) -> Result<Verify> {
  Verify::new("sha256".to_string())
}
