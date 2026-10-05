use napi::bindgen_prelude::*;
use napi_derive::napi;
use rsa::pkcs8::DecodePrivateKey;
use rsa::signature::{SignatureEncoding, Signer};
use rsa::RsaPrivateKey;
use sha2::Sha256;

use crate::hmac::{decode_input, encode_output};

#[napi(js_name = "Sign")]
pub struct Sign {
  pub algorithm: String,
  pub data: Vec<u8>,
}

#[napi]
impl Sign {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> Self {
    Sign {
      algorithm,
      data: Vec::new(),
    }
  }

  #[napi(ts_args_type = "data: string | Buffer, inputEncoding?: string")]
  pub fn update(&mut self, data: Either<String, Buffer>, encoding: Option<String>) {
    let bytes = decode_input(&data, encoding.as_deref());
    self.data.extend_from_slice(&bytes);
  }

  #[napi(ts_args_type = "key: string | Buffer, outputEncoding?: string", ts_return_type = "string | Buffer")]
  pub fn sign(&self, key: Either<String, Buffer>, output_encoding: Option<String>) -> napi::Result<Either<String, Buffer>> {
    let key_bytes = decode_input(&key, None);
    let key_str = String::from_utf8_lossy(&key_bytes);

    let rsa_key = RsaPrivateKey::from_pkcs8_pem(&key_str)
      .map_err(|e| napi::Error::from_reason(format!("Invalid RSA private key PEM: {}", e)))?;
    let signing_key = rsa::pkcs1v15::SigningKey::<Sha256>::new_unprefixed(rsa_key);
    let sig = signing_key.sign(&self.data);

    Ok(encode_output(&sig.to_vec(), output_encoding.as_deref()))
  }
}

#[napi]
pub fn create_sign(algorithm: String) -> Sign {
  Sign::new(algorithm)
}

#[napi(ts_args_type = "algorithm: string, data: string | Buffer, key: string | Buffer")]
pub fn sign(
  algorithm: String,
  data: Either<String, Buffer>,
  key: Either<String, Buffer>,
) -> napi::Result<Buffer> {
  let mut s = Sign::new(algorithm);
  s.update(data, None);
  let res = s.sign(key, None)?;
  match res {
    Either::A(str_val) => Ok(Buffer::from(str_val.as_bytes())),
    Either::B(buf) => Ok(buf),
  }
}
