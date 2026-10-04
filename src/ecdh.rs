use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::agreement::{self, EphemeralPrivateKey};
use ring::rand::SystemRandom;

#[napi]
pub struct Ecdh {
  public_key: Vec<u8>,
}

#[napi]
impl Ecdh {
  #[napi(constructor)]
  pub fn new(curve_name: String) -> Result<Self> {
    let rng = SystemRandom::new();
    let algorithm = match curve_name.to_lowercase().as_str() {
      "x25519" => &agreement::X25519,
      _ => &agreement::X25519,
    };

    let my_private_key = EphemeralPrivateKey::generate(algorithm, &rng)
      .map_err(|_| Error::from_reason("Failed to generate key"))?;

    let public_key = my_private_key
      .compute_public_key()
      .map_err(|_| Error::from_reason("Failed to compute public key"))?;

    Ok(Self {
      public_key: public_key.as_ref().to_vec(),
    })
  }

  #[napi]
  pub fn get_public_key(&self) -> Buffer {
    Buffer::from(self.public_key.clone())
  }
}

#[napi]
pub fn create_ecdh(curve_name: String) -> Result<Ecdh> {
  Ecdh::new(curve_name)
}
