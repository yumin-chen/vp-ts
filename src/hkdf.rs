use napi::bindgen_prelude::*;
use napi_derive::napi;
use hkdf::Hkdf;
use sha1::Sha1;
use sha2::{Sha224, Sha256, Sha384, Sha512};

fn run_hkdf(
  digest: &str,
  ikm: &[u8],
  salt: &[u8],
  info: &[u8],
  keylen: usize,
) -> Result<Vec<u8>> {
  let algo_clean = digest.to_lowercase().replace('-', "");
  let mut okm = vec![0u8; keylen];

  let res = match algo_clean.as_str() {
    "sha1" => {
      let hk = Hkdf::<Sha1>::new(Some(salt), ikm);
      hk.expand(info, &mut okm)
    }
    "sha224" => {
      let hk = Hkdf::<Sha224>::new(Some(salt), ikm);
      hk.expand(info, &mut okm)
    }
    "sha256" => {
      let hk = Hkdf::<Sha256>::new(Some(salt), ikm);
      hk.expand(info, &mut okm)
    }
    "sha384" => {
      let hk = Hkdf::<Sha384>::new(Some(salt), ikm);
      hk.expand(info, &mut okm)
    }
    "sha512" => {
      let hk = Hkdf::<Sha512>::new(Some(salt), ikm);
      hk.expand(info, &mut okm)
    }
    _ => return Err(Error::new(Status::InvalidArg, format!("Unsupported HKDF digest: {}", digest))),
  };

  res.map_err(|e| Error::new(Status::GenericFailure, format!("HKDF expand error: {:?}", e)))?;
  Ok(okm)
}

#[napi]
pub fn hkdf_sync(
  digest: String,
  ikm: Buffer,
  salt: Buffer,
  info: Buffer,
  keylen: u32,
) -> Result<Buffer> {
  let res = run_hkdf(&digest, &ikm, &salt, &info, keylen as usize)?;
  Ok(Buffer::from(res))
}

#[napi]
pub async fn hkdf(
  digest: String,
  ikm: Buffer,
  salt: Buffer,
  info: Buffer,
  keylen: u32,
) -> Result<Buffer> {
  let res = run_hkdf(&digest, &ikm, &salt, &info, keylen as usize)?;
  Ok(Buffer::from(res))
}
