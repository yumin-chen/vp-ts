use napi::bindgen_prelude::*;
use napi_derive::napi;
use rand::rngs::OsRng;
use x25519_dalek::{PublicKey, StaticSecret};

#[napi]
pub struct X25519DiffieHellman {
  secret: Option<StaticSecret>,
  public_key_bytes: Vec<u8>,
}

#[napi]
impl X25519DiffieHellman {
  #[napi(constructor)]
  pub fn new() -> Self {
    let secret = StaticSecret::random_from_rng(OsRng);
    let public = PublicKey::from(&secret);
    X25519DiffieHellman {
      secret: Some(secret),
      public_key_bytes: public.as_bytes().to_vec(),
    }
  }

  #[napi]
  pub fn get_public_key(&self) -> Buffer {
    Buffer::from(self.public_key_bytes.clone())
  }

  #[napi]
  pub fn compute_secret(&mut self, peer_public_key: Buffer) -> Result<Buffer> {
    let secret = self.secret.take().ok_or_else(|| {
      Error::new(Status::GenericFailure, "X25519 secret already consumed")
    })?;

    if peer_public_key.len() != 32 {
      return Err(Error::new(
        Status::InvalidArg,
        "Peer public key must be 32 bytes for X25519",
      ));
    }

    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&peer_public_key);
    let peer_public = PublicKey::from(bytes);

    let shared = secret.diffie_hellman(&peer_public);
    Ok(Buffer::from(shared.as_bytes().to_vec()))
  }
}
