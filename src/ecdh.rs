use napi::bindgen_prelude::*;
use napi_derive::napi;
use p256::elliptic_curve::sec1::ToSec1Point;
use p256::elliptic_curve::Generate;
use p256::{PublicKey as P256PublicKey, SecretKey as P256SecretKey};

#[napi(js_name = "ECDH")]
pub struct Ecdh {
  #[allow(dead_code)]
  curve: String,
  private_key: Option<Vec<u8>>,
  public_key: Option<Vec<u8>>,
}

#[napi]
impl Ecdh {
  #[napi(constructor)]
  pub fn new(curve: String) -> Result<Self> {
    if curve.to_lowercase() != "prime256v1" && curve.to_lowercase() != "p-256" && curve.to_lowercase() != "secp256r1" {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Unsupported curve: {}. Currently supported: prime256v1 / secp256r1", curve),
      ));
    }

    Ok(Self {
      curve,
      private_key: None,
      public_key: None,
    })
  }

  #[napi]
  pub fn generate_keys(&mut self) -> Result<Buffer> {
    let secret = P256SecretKey::generate();
    let pubkey = secret.public_key();

    let priv_bytes = secret.to_bytes().to_vec();
    let pub_bytes = pubkey.to_sec1_point(false).as_bytes().to_vec();

    self.private_key = Some(priv_bytes);
    self.public_key = Some(pub_bytes.clone());

    Ok(Buffer::from(pub_bytes))
  }

  #[napi]
  pub fn get_private_key(&self) -> Result<Buffer> {
    let k = self
      .private_key
      .as_ref()
      .ok_or_else(|| Error::new(Status::GenericFailure, "Private key not generated"))?;
    Ok(Buffer::from(k.clone()))
  }

  #[napi]
  pub fn get_public_key(&self) -> Result<Buffer> {
    let k = self
      .public_key
      .as_ref()
      .ok_or_else(|| Error::new(Status::GenericFailure, "Public key not generated"))?;
    Ok(Buffer::from(k.clone()))
  }

  #[napi]
  pub fn compute_secret(
    &self,
    #[napi(ts_arg_type = "Uint8Array")] other_public_key: Uint8Array,
  ) -> Result<Buffer> {
    let priv_bytes = self
      .private_key
      .as_ref()
      .ok_or_else(|| Error::new(Status::GenericFailure, "Private key not set"))?;

    let secret_key = P256SecretKey::from_slice(priv_bytes)
      .map_err(|_| Error::new(Status::InvalidArg, "Invalid private key"))?;

    let other_pub = P256PublicKey::from_sec1_bytes(other_public_key.as_ref())
      .map_err(|_| Error::new(Status::InvalidArg, "Invalid public key bytes"))?;

    let shared = p256::ecdh::diffie_hellman(secret_key.to_nonzero_scalar(), other_pub.as_affine());
    Ok(Buffer::from(shared.raw_secret_bytes().as_slice()))
  }
}

#[napi(js_name = "createECDH")]
pub fn create_ecdh(curve_name: String) -> Result<Ecdh> {
  Ecdh::new(curve_name)
}

#[napi]
pub fn create_diffie_hellman(group_name_or_prime_len: Either<String, u32>) -> Result<Ecdh> {
  let curve = match group_name_or_prime_len {
    Either::A(s) => s,
    Either::B(_) => "prime256v1".to_string(),
  };
  let mut ecdh = Ecdh::new(curve)?;
  let _ = ecdh.generate_keys()?;
  Ok(ecdh)
}

#[napi]
pub fn create_diffie_hellman_group(group_name: String) -> Result<Ecdh> {
  create_diffie_hellman(Either::A(group_name))
}
