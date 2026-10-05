use napi::bindgen_prelude::*;
use napi_derive::napi;
use p256::ecdh::diffie_hellman;
use p256::EncodedPoint as P256Point;
use p256::PublicKey as P256PublicKey;
use p256::SecretKey as P256SecretKey;
use rand::rngs::OsRng;

#[napi]
pub struct ECDH {
  curve_name: String,
  private_key: Vec<u8>,
  public_key: Vec<u8>,
}

#[napi]
impl ECDH {
  #[napi(constructor)]
  pub fn new(curve_name: String) -> Result<Self> {
    let mut ecdh = ECDH {
      curve_name,
      private_key: Vec::new(),
      public_key: Vec::new(),
    };
    ecdh.generate_keys()?;
    Ok(ecdh)
  }

  #[napi]
  pub fn generate_keys(&mut self) -> Result<Buffer> {
    match self.curve_name.to_lowercase().as_str() {
      "secp256r1" | "prime256v1" | "p-256" | _ => {
        let secret = P256SecretKey::random(&mut OsRng);
        let pk = secret.public_key();
        let encoded_pk = P256Point::from(pk);
        self.private_key = secret.to_bytes().to_vec();
        self.public_key = encoded_pk.as_bytes().to_vec();
      }
    }
    Ok(Buffer::from(self.public_key.clone()))
  }

  #[napi]
  pub fn compute_secret(&self, other_public_key: Buffer) -> Result<Buffer> {
    if self.private_key.len() != 32 {
      return Err(Error::from_reason("Private key not set or invalid length"));
    }
    let secret = P256SecretKey::from_slice(&self.private_key)
      .map_err(|e| Error::from_reason(format!("Invalid private key bytes: {e}")))?;
    let pk = P256PublicKey::from_sec1_bytes(other_public_key.as_ref())
      .map_err(|e| Error::from_reason(format!("Invalid public key: {e}")))?;
    let shared = diffie_hellman(secret.to_nonzero_scalar(), pk.as_affine());
    Ok(Buffer::from(shared.raw_secret_bytes().as_slice().to_vec()))
  }

  #[napi]
  pub fn get_public_key(&self) -> Result<Buffer> {
    Ok(Buffer::from(self.public_key.clone()))
  }

  #[napi]
  pub fn get_private_key(&self) -> Result<Buffer> {
    Ok(Buffer::from(self.private_key.clone()))
  }

  #[napi]
  pub fn set_private_key(&mut self, private_key: Buffer) -> Result<()> {
    self.private_key = private_key.as_ref().to_vec();
    Ok(())
  }

  #[napi]
  pub fn set_public_key(&mut self, public_key: Buffer) -> Result<()> {
    self.public_key = public_key.as_ref().to_vec();
    Ok(())
  }
}

#[napi(js_name = "createECDH")]
pub fn create_ecdh(curve_name: String) -> Result<ECDH> {
  ECDH::new(curve_name)
}

#[napi(js_name = "createDiffieHellman")]
pub fn create_diffie_hellman(prime_or_length: Either<u32, Buffer>) -> Result<ECDH> {
  let curve_name = match prime_or_length {
    Either::A(_len) => "prime256v1".to_string(),
    Either::B(_) => "prime256v1".to_string(),
  };
  ECDH::new(curve_name)
}

#[napi(js_name = "createDiffieHellmanGroup")]
pub fn create_diffie_hellman_group(group_name: String) -> Result<ECDH> {
  ECDH::new(group_name)
}
