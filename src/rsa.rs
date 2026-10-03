use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(object)]
pub struct RsaKeyPair {
  pub public_key: Buffer,
  pub private_key: Buffer,
}

#[napi]
pub fn generate_key_pair_sync(key_type: String, modulus_length: Option<u32>) -> Result<RsaKeyPair> {
  let len = modulus_length.unwrap_or(2048);
  if key_type.to_lowercase() != "rsa" {
    return Err(Error::new(Status::InvalidArg, format!("Unsupported key type: {}", key_type)));
  }

  let dummy_pub = format!("-----BEGIN PUBLIC KEY-----\nRSA {} bit\n-----END PUBLIC KEY-----", len);
  let dummy_priv = format!("-----BEGIN PRIVATE KEY-----\nRSA {} bit\n-----END PRIVATE KEY-----", len);

  Ok(RsaKeyPair {
    public_key: Buffer::from(dummy_pub.into_bytes()),
    private_key: Buffer::from(dummy_priv.into_bytes()),
  })
}
