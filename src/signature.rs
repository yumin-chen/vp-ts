use napi::bindgen_prelude::*;
use napi_derive::napi;
use rsa::{
  pkcs8::{DecodePrivateKey, DecodePublicKey},
  Pkcs1v15Sign, RsaPrivateKey, RsaPublicKey,
};
use sha2::{Digest, Sha256, Sha384, Sha512};

#[napi]
pub struct Sign {
  algorithm: String,
  data: Vec<u8>,
}

#[napi]
impl Sign {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> Self {
    Self {
      algorithm,
      data: Vec::new(),
    }
  }

  #[napi]
  pub fn update(&mut self, data: Buffer) {
    self.data.extend_from_slice(&data);
  }

  #[napi]
  pub fn sign(&self, private_key_pem: String) -> Result<Buffer> {
    let priv_key = RsaPrivateKey::from_pkcs8_pem(&private_key_pem)
      .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid private key: {}", e)))?;

    let algo_clean = self.algorithm.to_lowercase().replace('-', "");
    let signature = match algo_clean.as_str() {
      "sha256" | "rsasha256" => {
        let digest = Sha256::digest(&self.data);
        priv_key
          .sign(Pkcs1v15Sign::new::<Sha256>(), &digest)
          .map_err(|e| Error::new(Status::GenericFailure, format!("Signing error: {}", e)))?
      }
      "sha384" | "rsasha384" => {
        let digest = Sha384::digest(&self.data);
        priv_key
          .sign(Pkcs1v15Sign::new::<Sha384>(), &digest)
          .map_err(|e| Error::new(Status::GenericFailure, format!("Signing error: {}", e)))?
      }
      "sha512" | "rsasha512" => {
        let digest = Sha512::digest(&self.data);
        priv_key
          .sign(Pkcs1v15Sign::new::<Sha512>(), &digest)
          .map_err(|e| Error::new(Status::GenericFailure, format!("Signing error: {}", e)))?
      }
      _ => return Err(Error::new(Status::InvalidArg, format!("Unsupported signing algorithm: {}", self.algorithm))),
    };

    Ok(Buffer::from(signature))
  }
}

#[napi]
pub fn create_sign(algorithm: String) -> Sign {
  Sign::new(algorithm)
}

#[napi]
pub fn sign(algorithm: String, data: Buffer, private_key_pem: String) -> Result<Buffer> {
  let mut s = Sign::new(algorithm);
  s.update(data);
  s.sign(private_key_pem)
}

#[napi]
pub struct Verify {
  algorithm: String,
  data: Vec<u8>,
}

#[napi]
impl Verify {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> Self {
    Self {
      algorithm,
      data: Vec::new(),
    }
  }

  #[napi]
  pub fn update(&mut self, data: Buffer) {
    self.data.extend_from_slice(&data);
  }

  #[napi]
  pub fn verify(&self, public_key_pem: String, signature: Buffer) -> Result<bool> {
    let pub_key = RsaPublicKey::from_public_key_pem(&public_key_pem)
      .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid public key: {}", e)))?;

    let algo_clean = self.algorithm.to_lowercase().replace('-', "");
    let valid = match algo_clean.as_str() {
      "sha256" | "rsasha256" => {
        let digest = Sha256::digest(&self.data);
        pub_key.verify(Pkcs1v15Sign::new::<Sha256>(), &digest, &signature).is_ok()
      }
      "sha384" | "rsasha384" => {
        let digest = Sha384::digest(&self.data);
        pub_key.verify(Pkcs1v15Sign::new::<Sha384>(), &digest, &signature).is_ok()
      }
      "sha512" | "rsasha512" => {
        let digest = Sha512::digest(&self.data);
        pub_key.verify(Pkcs1v15Sign::new::<Sha512>(), &digest, &signature).is_ok()
      }
      _ => return Err(Error::new(Status::InvalidArg, format!("Unsupported verification algorithm: {}", self.algorithm))),
    };

    Ok(valid)
  }
}

#[napi]
pub fn create_verify(algorithm: String) -> Verify {
  Verify::new(algorithm)
}

#[napi]
pub fn verify(
  algorithm: String,
  data: Buffer,
  public_key_pem: String,
  signature: Buffer,
) -> Result<bool> {
  let mut v = Verify::new(algorithm);
  v.update(data);
  v.verify(public_key_pem, signature)
}
