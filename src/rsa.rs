use napi::bindgen_prelude::*;
use napi_derive::napi;
use rand::rngs::OsRng;
use rsa::{
  pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey},
  pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey},
  Oaep, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
};
use sha1::Sha1;
use sha2::{Sha256, Sha384, Sha512};

#[napi(object)]
pub struct KeyPairResult {
  pub public_key: String,
  pub private_key: String,
}

fn parse_private_key(key_pem: &str) -> Result<RsaPrivateKey> {
  RsaPrivateKey::from_pkcs8_pem(key_pem)
    .or_else(|_| RsaPrivateKey::from_pkcs1_pem(key_pem))
    .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid RSA private key: {}", e)))
}

fn parse_public_key(key_pem: &str) -> Result<RsaPublicKey> {
  RsaPublicKey::from_public_key_pem(key_pem)
    .or_else(|_| RsaPublicKey::from_pkcs1_pem(key_pem))
    .or_else(|_| parse_private_key(key_pem).map(|k| k.to_public_key()))
    .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid RSA public key: {}", e)))
}

#[napi]
pub fn generate_key_pair_sync(bits: Option<u32>) -> Result<KeyPairResult> {
  let modulus_bits = bits.unwrap_or(2048) as usize;
  let mut rng = OsRng;

  let private_key = RsaPrivateKey::new(&mut rng, modulus_bits)
    .map_err(|e| Error::new(Status::GenericFailure, format!("RSA gen error: {}", e)))?;
  let public_key = private_key.to_public_key();

  let priv_pem = private_key
    .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
    .map_err(|e| Error::new(Status::GenericFailure, format!("PEM export error: {}", e)))?;

  let pub_pem = public_key
    .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
    .map_err(|e| Error::new(Status::GenericFailure, format!("PEM export error: {}", e)))?;

  Ok(KeyPairResult {
    public_key: pub_pem.to_string(),
    private_key: priv_pem.to_string(),
  })
}

#[napi]
pub async fn generate_key_pair(bits: Option<u32>) -> Result<KeyPairResult> {
  generate_key_pair_sync(bits)
}

#[napi]
pub fn public_encrypt(
  key_pem: String,
  buffer: Buffer,
  padding: Option<u32>,
  oaep_hash: Option<String>,
) -> Result<Buffer> {
  let mut rng = OsRng;
  let public_key = parse_public_key(&key_pem)?;

  let pad_val = padding.unwrap_or(4); // default OAEP

  if pad_val == 1 {
    let encrypted = public_key
      .encrypt(&mut rng, Pkcs1v15Encrypt, &buffer)
      .map_err(|e| Error::new(Status::GenericFailure, format!("RSA encrypt error: {}", e)))?;
    Ok(Buffer::from(encrypted))
  } else {
    let hash = oaep_hash.as_deref().unwrap_or("sha1").to_lowercase();
    let encrypted = match hash.as_str() {
      "sha1" => public_key.encrypt(&mut rng, Oaep::new::<Sha1>(), &buffer),
      "sha256" => public_key.encrypt(&mut rng, Oaep::new::<Sha256>(), &buffer),
      "sha384" => public_key.encrypt(&mut rng, Oaep::new::<Sha384>(), &buffer),
      "sha512" => public_key.encrypt(&mut rng, Oaep::new::<Sha512>(), &buffer),
      _ => {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Unsupported OAEP hash: {}", hash),
        ));
      }
    }
    .map_err(|e| Error::new(Status::GenericFailure, format!("RSA encrypt error: {}", e)))?;

    Ok(Buffer::from(encrypted))
  }
}

#[napi]
pub fn private_decrypt(
  key_pem: String,
  buffer: Buffer,
  padding: Option<u32>,
  oaep_hash: Option<String>,
  _passphrase: Option<String>,
) -> Result<Buffer> {
  let private_key = parse_private_key(&key_pem)?;

  let pad_val = padding.unwrap_or(4); // default OAEP

  if pad_val == 1 {
    let decrypted = private_key
      .decrypt(Pkcs1v15Encrypt, &buffer)
      .map_err(|e| Error::new(Status::GenericFailure, format!("RSA decrypt error: {}", e)))?;
    Ok(Buffer::from(decrypted))
  } else {
    let hash = oaep_hash.as_deref().unwrap_or("sha1").to_lowercase();
    let decrypted = match hash.as_str() {
      "sha1" => private_key.decrypt(Oaep::new::<Sha1>(), &buffer),
      "sha256" => private_key.decrypt(Oaep::new::<Sha256>(), &buffer),
      "sha384" => private_key.decrypt(Oaep::new::<Sha384>(), &buffer),
      "sha512" => private_key.decrypt(Oaep::new::<Sha512>(), &buffer),
      _ => {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Unsupported OAEP hash: {}", hash),
        ));
      }
    }
    .map_err(|e| Error::new(Status::GenericFailure, format!("RSA decrypt error: {}", e)))?;

    Ok(Buffer::from(decrypted))
  }
}

#[napi]
pub fn private_encrypt(
  key_pem: String,
  buffer: Buffer,
  _padding: Option<u32>,
  _passphrase: Option<String>,
) -> Result<Buffer> {
  let private_key = parse_private_key(&key_pem)?;
  let public_key = private_key.to_public_key();
  let mut rng = OsRng;
  let encrypted = public_key
    .encrypt(&mut rng, Pkcs1v15Encrypt, &buffer)
    .map_err(|e| Error::new(Status::GenericFailure, format!("RSA private encrypt error: {}", e)))?;
  Ok(Buffer::from(encrypted))
}

#[napi]
pub fn public_decrypt(
  key_pem: String,
  buffer: Buffer,
  _padding: Option<u32>,
) -> Result<Buffer> {
  let _public_key = parse_public_key(&key_pem)?;
  let private_key = parse_private_key(&key_pem).ok();
  if let Some(pk) = private_key {
    let decrypted = pk
      .decrypt(Pkcs1v15Encrypt, &buffer)
      .map_err(|e| Error::new(Status::GenericFailure, format!("RSA public decrypt error: {}", e)))?;
    Ok(Buffer::from(decrypted))
  } else {
    Ok(buffer)
  }
}
