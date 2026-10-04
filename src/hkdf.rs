use hkdf::Hkdf;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use sha2::{Sha256, Sha384, Sha512};

#[napi(js_name = "hkdfSync")]
pub fn hkdf_sync(
  digest: String,
  ikm: Either<Buffer, String>,
  salt: Either<Buffer, String>,
  info: Either<Buffer, String>,
  keylen: u32,
) -> Result<Buffer> {
  let ikm_bytes = match ikm {
    Either::A(b) => b.as_ref().to_vec(),
    Either::B(s) => s.as_bytes().to_vec(),
  };
  let salt_bytes = match salt {
    Either::A(b) => b.as_ref().to_vec(),
    Either::B(s) => s.as_bytes().to_vec(),
  };
  let info_bytes = match info {
    Either::A(b) => b.as_ref().to_vec(),
    Either::B(s) => s.as_bytes().to_vec(),
  };

  let alg = digest.to_lowercase().replace('-', "");
  let mut okm = vec![0u8; keylen as usize];

  match alg.as_str() {
    "sha256" => {
      let hk = Hkdf::<Sha256>::new(Some(&salt_bytes), &ikm_bytes);
      hk.expand(&info_bytes, &mut okm)
        .map_err(|e| Error::from_reason(format!("HKDF expand error: {e}")))?;
    }
    "sha384" => {
      let hk = Hkdf::<Sha384>::new(Some(&salt_bytes), &ikm_bytes);
      hk.expand(&info_bytes, &mut okm)
        .map_err(|e| Error::from_reason(format!("HKDF expand error: {e}")))?;
    }
    "sha512" => {
      let hk = Hkdf::<Sha512>::new(Some(&salt_bytes), &ikm_bytes);
      hk.expand(&info_bytes, &mut okm)
        .map_err(|e| Error::from_reason(format!("HKDF expand error: {e}")))?;
    }
    other => return Err(Error::from_reason(format!("Unsupported HKDF digest: {other}"))),
  }

  Ok(Buffer::from(okm))
}

#[napi(js_name = "hkdf")]
pub fn hkdf(
  digest: String,
  ikm: Either<Buffer, String>,
  salt: Either<Buffer, String>,
  info: Either<Buffer, String>,
  keylen: u32,
) -> Result<Buffer> {
  hkdf_sync(digest, ikm, salt, info, keylen)
}
