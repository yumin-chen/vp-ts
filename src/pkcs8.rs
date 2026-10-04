use napi_derive::napi;
use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey};

#[napi]
pub fn validate_pkcs8_private_key(pem: String) -> bool {
  rsa::RsaPrivateKey::from_pkcs8_pem(&pem).is_ok()
}

#[napi]
pub fn validate_spki_public_key(pem: String) -> bool {
  rsa::RsaPublicKey::from_public_key_pem(&pem).is_ok()
}
