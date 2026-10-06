use napi::bindgen_prelude::*;
use napi_derive::napi;
use rand::rngs::OsRng;
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey};
use rsa::{Oaep, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use sha1::Sha1;
use sha2::{Sha256, Sha384, Sha512};

#[napi(object)]
pub struct RsaOptions {
  pub key: Either<String, Buffer>,
  pub passphrase: Option<Either<String, Buffer>>,
  pub padding: Option<u32>,
  pub oaep_hash: Option<String>,
}

fn parse_pem_string(input: Either<String, Buffer>) -> String {
  match input {
    Either::A(s) => s,
    Either::B(b) => String::from_utf8_lossy(b.as_ref()).to_string(),
  }
}

#[napi]
pub fn public_encrypt(
  key: Either<String, RsaOptions>,
  buffer: Buffer,
) -> Result<Buffer> {
  let (pem, oaep_hash, _passphrase) = match key {
    Either::A(s) => (s, None, None),
    Either::B(opts) => {
      let k = parse_pem_string(opts.key);
      let pass = opts.passphrase.map(parse_pem_string);
      (k, opts.oaep_hash, pass)
    }
  };

  let public_key = RsaPublicKey::from_public_key_pem(&pem)
    .or_else(|_| RsaPrivateKey::from_pkcs8_pem(&pem).map(|pk| RsaPublicKey::from(&pk)))
    .or_else(|_| RsaPrivateKey::from_pkcs1_pem(&pem).map(|pk| RsaPublicKey::from(&pk)))
    .map_err(|e| Error::new(Status::InvalidArg, format!("ERR_INVALID_ARG_VALUE: {e}")))?;

  let mut rng = OsRng;
  let ciphertext = match oaep_hash.as_deref() {
    Some("sha256") => public_key
      .encrypt(&mut rng, Oaep::new::<Sha256>(), buffer.as_ref())
      .map_err(|e: rsa::Error| Error::from_reason(e.to_string()))?,
    Some("sha384") => public_key
      .encrypt(&mut rng, Oaep::new::<Sha384>(), buffer.as_ref())
      .map_err(|e: rsa::Error| Error::from_reason(e.to_string()))?,
    Some("sha512") => public_key
      .encrypt(&mut rng, Oaep::new::<Sha512>(), buffer.as_ref())
      .map_err(|e: rsa::Error| Error::from_reason(e.to_string()))?,
    Some("sha1") | None => public_key
      .encrypt(&mut rng, Oaep::new::<Sha1>(), buffer.as_ref())
      .or_else(|_| public_key.encrypt(&mut rng, Pkcs1v15Encrypt, buffer.as_ref()))
      .map_err(|e: rsa::Error| Error::from_reason(e.to_string()))?,
    Some(other) => return Err(Error::new(Status::InvalidArg, format!("ERR_OSSL_EVP_INVALID_DIGEST: {other}"))),
  };

  Ok(Buffer::from(ciphertext))
}

#[napi]
pub fn private_decrypt(
  key: Either<String, RsaOptions>,
  buffer: Buffer,
) -> Result<Buffer> {
  let (pem, oaep_hash, _passphrase) = match key {
    Either::A(s) => (s, None, None),
    Either::B(opts) => {
      let k = parse_pem_string(opts.key);
      let pass = opts.passphrase.map(parse_pem_string);
      (k, opts.oaep_hash, pass)
    }
  };

  let private_key = RsaPrivateKey::from_pkcs8_pem(&pem)
    .or_else(|_| RsaPrivateKey::from_pkcs1_pem(&pem))
    .map_err(|e| Error::new(Status::InvalidArg, format!("ERR_INVALID_ARG_VALUE: {e}")))?;

  let plaintext = match oaep_hash.as_deref() {
    Some("sha256") => private_key
      .decrypt(Oaep::new::<Sha256>(), buffer.as_ref())
      .map_err(|e: rsa::Error| Error::from_reason(e.to_string()))?,
    Some("sha384") => private_key
      .decrypt(Oaep::new::<Sha384>(), buffer.as_ref())
      .map_err(|e: rsa::Error| Error::from_reason(e.to_string()))?,
    Some("sha512") => private_key
      .decrypt(Oaep::new::<Sha512>(), buffer.as_ref())
      .map_err(|e: rsa::Error| Error::from_reason(e.to_string()))?,
    Some("sha1") | None => private_key
      .decrypt(Oaep::new::<Sha1>(), buffer.as_ref())
      .or_else(|_| private_key.decrypt(Pkcs1v15Encrypt, buffer.as_ref()))
      .map_err(|e: rsa::Error| Error::from_reason(e.to_string()))?,
    Some(other) => return Err(Error::new(Status::InvalidArg, format!("ERR_OSSL_EVP_INVALID_DIGEST: {other}"))),
  };

  Ok(Buffer::from(plaintext))
}

#[napi]
pub fn public_decrypt(
  key: Either<String, RsaOptions>,
  buffer: Buffer,
) -> Result<Buffer> {
  private_decrypt(key, buffer)
}

#[napi]
pub fn private_encrypt(
  key: Either<String, RsaOptions>,
  buffer: Buffer,
) -> Result<Buffer> {
  public_encrypt(key, buffer)
}
