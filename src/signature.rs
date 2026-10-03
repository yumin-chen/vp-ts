use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::signature as ring_sig;

#[napi]
pub struct Sign {
  #[allow(dead_code)]
  algorithm: String,
  data: Vec<u8>,
}

#[napi]
impl Sign {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> Self {
    Self {
      algorithm,
      data: Vec::new(),
    }
  }

  #[napi]
  pub fn update(
    &mut self,
    #[napi(ts_arg_type = "string | Uint8Array")] data: Either<String, Uint8Array>,
  ) {
    match data {
      Either::A(s) => self.data.extend_from_slice(s.as_bytes()),
      Either::B(b) => self.data.extend_from_slice(b.as_ref()),
    }
  }

  #[napi]
  pub fn sign(
    &self,
    #[napi(ts_arg_type = "Uint8Array")] private_key_pkcs8: Uint8Array,
    output_encoding: Option<String>,
  ) -> Result<Either<Buffer, String>> {
    let key_pair = ring_sig::Ed25519KeyPair::from_pkcs8(private_key_pkcs8.as_ref())
      .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid Ed25519 PKCS#8 key: {}", e)))?;

    let sig = key_pair.sign(&self.data);
    let bytes = sig.as_ref();

    match output_encoding.as_deref() {
      Some("hex") => Ok(Either::B(hex::encode(bytes))),
      Some("base64") => {
        use base64::Engine;
        Ok(Either::B(
          base64::engine::general_purpose::STANDARD.encode(bytes),
        ))
      }
      Some("buffer") | None => Ok(Either::A(Buffer::from(bytes))),
      Some(other) => Err(Error::new(
        Status::InvalidArg,
        format!("Unsupported encoding: {}", other),
      )),
    }
  }
}

#[napi]
pub fn create_sign(algorithm: String) -> Sign {
  Sign::new(algorithm)
}

#[napi]
pub fn sign(
  algorithm: String,
  #[napi(ts_arg_type = "string | Uint8Array")] data: Either<String, Uint8Array>,
  #[napi(ts_arg_type = "Uint8Array")] private_key: Uint8Array,
) -> Result<Buffer> {
  let mut s = Sign::new(algorithm);
  s.update(data);
  let res = s.sign(private_key, None)?;
  match res {
    Either::A(b) => Ok(b),
    Either::B(_) => unreachable!(),
  }
}
