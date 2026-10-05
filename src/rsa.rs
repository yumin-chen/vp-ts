use napi::bindgen_prelude::*;
use napi_derive::napi;
use num_bigint_dig::BigUint;
use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey};
use rsa::traits::{PrivateKeyParts, PublicKeyParts};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

use crate::hmac::decode_input;
use crate::key_object::CryptoKeyPair;

#[napi]
pub fn generate_key_pair_sync(type_name: String, modulus_length: Option<u32>) -> napi::Result<CryptoKeyPair> {
  let bits = modulus_length.unwrap_or(2048) as usize;
  let mut rng = rand::thread_rng();

  let priv_key = RsaPrivateKey::new(&mut rng, bits)
    .map_err(|e| napi::Error::from_reason(format!("RSA keygen failed: {}", e)))?;
  let pub_key = RsaPublicKey::from(&priv_key);

  let priv_pem = priv_key
    .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
    .map_err(|e| napi::Error::from_reason(format!("PEM export failed: {}", e)))?;
  let pub_pem = pub_key
    .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
    .map_err(|e| napi::Error::from_reason(format!("PEM export failed: {}", e)))?;

  Ok(CryptoKeyPair {
    pub_type: "public".to_string(),
    pub_asym_type: Some(type_name.clone()),
    pub_bytes: pub_pem.as_bytes().to_vec(),
    priv_type: "private".to_string(),
    priv_asym_type: Some(type_name),
    priv_bytes: priv_pem.as_bytes().to_vec(),
  })
}

#[napi(ts_args_type = "key: string | Buffer, buffer: string | Buffer")]
pub fn public_encrypt(key: Either<String, Buffer>, buffer: Either<String, Buffer>) -> napi::Result<Buffer> {
  let key_bytes = decode_input(&key, None);
  let data_bytes = decode_input(&buffer, None);

  let key_str = String::from_utf8_lossy(&key_bytes);
  let pub_key = RsaPublicKey::from_public_key_pem(&key_str)
    .map_err(|e| napi::Error::from_reason(format!("Invalid public key PEM: {}", e)))?;

  let mut rng = rand::thread_rng();
  let encrypted = pub_key
    .encrypt(&mut rng, Pkcs1v15Encrypt, &data_bytes)
    .map_err(|e| napi::Error::from_reason(format!("RSA encryption failed: {}", e)))?;

  Ok(Buffer::from(encrypted))
}

#[napi(ts_args_type = "key: string | Buffer, buffer: string | Buffer")]
pub fn private_decrypt(key: Either<String, Buffer>, buffer: Either<String, Buffer>) -> napi::Result<Buffer> {
  let key_bytes = decode_input(&key, None);
  let data_bytes = decode_input(&buffer, None);

  let key_str = String::from_utf8_lossy(&key_bytes);
  let priv_key = RsaPrivateKey::from_pkcs8_pem(&key_str)
    .map_err(|e| napi::Error::from_reason(format!("Invalid private key PEM: {}", e)))?;

  let decrypted = priv_key
    .decrypt(Pkcs1v15Encrypt, &data_bytes)
    .map_err(|e| napi::Error::from_reason(format!("RSA decryption failed: {}", e)))?;

  Ok(Buffer::from(decrypted))
}

#[napi(ts_args_type = "key: string | Buffer, buffer: string | Buffer")]
pub fn private_encrypt(key: Either<String, Buffer>, buffer: Either<String, Buffer>) -> napi::Result<Buffer> {
  let key_bytes = decode_input(&key, None);
  let data_bytes = decode_input(&buffer, None);

  let key_str = String::from_utf8_lossy(&key_bytes);
  let priv_key = RsaPrivateKey::from_pkcs8_pem(&key_str)
    .map_err(|e| napi::Error::from_reason(format!("Invalid private key PEM: {}", e)))?;

  let m = BigUint::from_bytes_be(&data_bytes);
  let c = m.modpow(priv_key.d(), priv_key.n());

  Ok(Buffer::from(c.to_bytes_be()))
}

#[napi(ts_args_type = "key: string | Buffer, buffer: string | Buffer")]
pub fn public_decrypt(key: Either<String, Buffer>, buffer: Either<String, Buffer>) -> napi::Result<Buffer> {
  let key_bytes = decode_input(&key, None);
  let data_bytes = decode_input(&buffer, None);

  let key_str = String::from_utf8_lossy(&key_bytes);
  let pub_key = RsaPublicKey::from_public_key_pem(&key_str)
    .map_err(|e| napi::Error::from_reason(format!("Invalid public key PEM: {}", e)))?;

  let c = BigUint::from_bytes_be(&data_bytes);
  let m = c.modpow(pub_key.e(), pub_key.n());

  Ok(Buffer::from(m.to_bytes_be()))
}
