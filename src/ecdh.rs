use napi::bindgen_prelude::*;
use napi_derive::napi;
use p256::ecdh::EphemeralSecret;
use rand::rngs::OsRng;

#[napi]
pub struct Ecdh {
  secret: Option<EphemeralSecret>,
  public_key_bytes: Vec<u8>,
}

#[napi]
impl Ecdh {
  #[napi(constructor)]
  pub fn new(curve_name: String) -> Result<Self> {
    if curve_name.to_lowercase() != "prime256v1" && curve_name.to_lowercase() != "p-256" {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Unsupported curve: {}", curve_name),
      ));
    }

    let secret = EphemeralSecret::random(&mut OsRng);
    let public_key = secret.public_key();
    let public_key_bytes = public_key.to_sec1_bytes().to_vec();

    Ok(Ecdh {
      secret: Some(secret),
      public_key_bytes,
    })
  }

  #[napi]
  pub fn generate_keys(&mut self) -> Buffer {
    Buffer::from(self.public_key_bytes.clone())
  }

  #[napi]
  pub fn get_public_key(&self) -> Buffer {
    Buffer::from(self.public_key_bytes.clone())
  }

  #[napi]
  pub fn compute_secret(&mut self, other_public_key: Buffer) -> Result<Buffer> {
    let secret = self.secret.take().ok_or_else(|| {
      Error::new(
        Status::GenericFailure,
        "Secret key already consumed for ECDH computation",
      )
    })?;

    let pk = p256::PublicKey::from_sec1_bytes(&other_public_key).map_err(|e| {
      Error::new(
        Status::InvalidArg,
        format!("Invalid public key SEC1 bytes: {}", e),
      )
    })?;

    let shared_secret = secret.diffie_hellman(&pk);
    Ok(Buffer::from(shared_secret.raw_secret_bytes().as_slice()))
  }
}

#[napi]
pub fn create_ecdh(curve_name: String) -> Result<Ecdh> {
  Ecdh::new(curve_name)
}
