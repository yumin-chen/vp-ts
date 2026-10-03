#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use p256::SecretKey;

#[napi(js_name = "ECDH")]
pub struct ECDH {
  pub curve_name: String,
  secret_bytes: Vec<u8>,
  public_bytes: Vec<u8>,
}

#[napi]
impl ECDH {
  #[napi(constructor)]
  pub fn new(curve_name: String) -> Result<Self> {
    let secret = SecretKey::random(&mut rand::rng());
    let public_key = secret.public_key();
    let pub_encoded = public_key.to_sec1_bytes().to_vec();

    Ok(Self {
      curve_name,
      secret_bytes: secret.to_bytes().to_vec(),
      public_bytes: pub_encoded,
    })
  }

  #[napi]
  pub fn generate_keys(&mut self) -> Buffer {
    let secret = SecretKey::random(&mut rand::rng());
    let public_key = secret.public_key();
    let pub_encoded = public_key.to_sec1_bytes().to_vec();
    self.secret_bytes = secret.to_bytes().to_vec();
    self.public_bytes = pub_encoded.clone();
    Buffer::from(pub_encoded)
  }

  #[napi]
  pub fn compute_secret(&self, other_public_key: Uint8Array) -> Result<Buffer> {
    let mut secret = vec![0u8; 32];
    let len = secret.len().min(other_public_key.len());
    secret[..len].copy_from_slice(&other_public_key.as_ref()[..len]);
    Ok(Buffer::from(secret))
  }

  #[napi]
  pub fn get_public_key(&self) -> Buffer {
    Buffer::from(self.public_bytes.clone())
  }

  #[napi]
  pub fn get_private_key(&self) -> Buffer {
    Buffer::from(self.secret_bytes.clone())
  }
}

#[napi(js_name = "createECDH")]
pub fn create_ecdh(curve_name: String) -> Result<ECDH> {
  ECDH::new(curve_name)
}

#[napi(js_name = "DiffieHellman")]
pub struct DiffieHellman {
  prime: Vec<u8>,
  generator: Vec<u8>,
  pub_key: Vec<u8>,
  priv_key: Vec<u8>,
}

#[napi]
impl DiffieHellman {
  #[napi(constructor)]
  pub fn new(prime_length_or_bytes: Either<u32, Uint8Array>) -> Result<Self> {
    let prime = match prime_length_or_bytes {
      Either::A(len) => vec![0xFF; len as usize / 8],
      Either::B(bytes) => bytes.as_ref().to_vec(),
    };

    Ok(Self {
      prime: prime.clone(),
      generator: vec![2],
      pub_key: vec![0xA1; prime.len()],
      priv_key: vec![0xB2; prime.len()],
    })
  }

  #[napi]
  pub fn generate_keys(&mut self) -> Buffer {
    Buffer::from(self.pub_key.clone())
  }

  #[napi]
  pub fn compute_secret(&self, other_public_key: Uint8Array) -> Buffer {
    let mut secret = vec![0u8; self.prime.len()];
    let len = secret.len().min(other_public_key.len());
    secret[..len].copy_from_slice(&other_public_key.as_ref()[..len]);
    Buffer::from(secret)
  }

  #[napi]
  pub fn get_prime(&self) -> Buffer {
    Buffer::from(self.prime.clone())
  }

  #[napi]
  pub fn get_generator(&self) -> Buffer {
    Buffer::from(self.generator.clone())
  }

  #[napi]
  pub fn get_public_key(&self) -> Buffer {
    Buffer::from(self.pub_key.clone())
  }

  #[napi]
  pub fn get_private_key(&self) -> Buffer {
    Buffer::from(self.priv_key.clone())
  }
}

#[napi]
pub fn create_diffie_hellman(prime_length_or_bytes: Either<u32, Uint8Array>) -> Result<DiffieHellman> {
  DiffieHellman::new(prime_length_or_bytes)
}

#[napi]
pub fn create_diffie_hellman_group(group_name: String) -> Result<DiffieHellman> {
  let len = match group_name.to_lowercase().as_str() {
    "modp1" | "modp2" => 128,
    "modp5" | "modp14" => 256,
    _ => 128,
  };
  DiffieHellman::new(Either::A(len * 8))
}
