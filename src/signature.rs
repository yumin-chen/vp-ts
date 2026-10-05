use napi::bindgen_prelude::*;
use napi_derive::napi;
use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey};
use rsa::{Pkcs1v15Sign, RsaPrivateKey, RsaPublicKey};
use sha2::{Digest, Sha256};

#[napi]
pub struct Sign {
  algorithm: String,
  data: Vec<u8>,
}

#[napi]
impl Sign {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> Self {
    Sign {
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
  pub fn sign(&mut self, key_pem: String) -> Result<Buffer> {
    let priv_key = RsaPrivateKey::from_pkcs8_pem(&key_pem)
      .map_err(|e| Error::from_reason(format!("Failed to parse private key: {e}")))?;

    let mut hasher = Sha256::new();
    hasher.update(&self.data);
    let hashed = hasher.finalize();

    let sig = priv_key
      .sign(Pkcs1v15Sign::new::<Sha256>(), &hashed)
      .map_err(|e| Error::from_reason(format!("Signing error: {e}")))?;

    Ok(Buffer::from(sig))
  }
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
    Verify {
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
  pub fn verify(&mut self, key_pem: String, signature: Buffer) -> Result<bool> {
    let pub_key = RsaPublicKey::from_public_key_pem(&key_pem)
      .map_err(|e| Error::from_reason(format!("Failed to parse public key: {e}")))?;

    let mut hasher = Sha256::new();
    hasher.update(&self.data);
    let hashed = hasher.finalize();

    let res = pub_key.verify(Pkcs1v15Sign::new::<Sha256>(), &hashed, signature.as_ref());
    Ok(res.is_ok())
  }
}

#[napi(js_name = "createSign")]
pub fn create_sign(algorithm: String) -> Sign {
  Sign::new(algorithm)
}

#[napi(js_name = "createVerify")]
pub fn create_verify(algorithm: String) -> Verify {
  Verify::new(algorithm)
}

#[napi(js_name = "sign")]
pub fn sign(algorithm: String, data: Either<Buffer, String>, key_pem: String) -> Result<Buffer> {
  let mut s = Sign::new(algorithm);
  s.update(data)?;
  s.sign(key_pem)
}

#[napi(js_name = "verify")]
pub fn verify(
  algorithm: String,
  data: Either<Buffer, String>,
  key_pem: String,
  signature: Buffer,
) -> Result<bool> {
  let mut v = Verify::new(algorithm);
  v.update(data)?;
  v.verify(key_pem, signature)
}
