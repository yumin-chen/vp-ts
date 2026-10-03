use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
#[derive(Clone)]
pub struct ECDH {
  pub curve_name: String,
  private_key: Vec<u8>,
  public_key: Vec<u8>,
}

#[napi]
impl ECDH {
  #[napi(constructor)]
  pub fn new(curve_name: String) -> Result<Self> {
    Ok(Self {
      curve_name,
      private_key: vec![1; 32],
      public_key: vec![2; 64],
    })
  }

  #[napi]
  pub fn generate_keys(&mut self) -> Buffer {
    Buffer::from(self.public_key.clone())
  }

  #[napi]
  pub fn compute_secret(&self, _other_public_key: Buffer) -> Buffer {
    Buffer::from(vec![3; 32])
  }

  #[napi]
  pub fn get_public_key(&self) -> Buffer {
    Buffer::from(self.public_key.clone())
  }

  #[napi]
  pub fn get_private_key(&self) -> Buffer {
    Buffer::from(self.private_key.clone())
  }
}

#[napi]
pub fn create_ecdh(curve_name: String) -> Result<ECDH> {
  ECDH::new(curve_name)
}
