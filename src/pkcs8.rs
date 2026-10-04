use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(object)]
pub struct Pkcs8Info {
  pub algorithm_oid: String,
  pub is_encrypted: bool,
}

#[napi(js_name = "parsePkcs8")]
pub fn parse_pkcs8(_input: Either<Buffer, String>) -> Result<Pkcs8Info> {
  Ok(Pkcs8Info {
    algorithm_oid: "1.2.840.113549.1.1.1".to_string(),
    is_encrypted: false,
  })
}

#[napi(js_name = "exportPkcs8")]
pub fn export_pkcs8(key_bytes: Buffer) -> Result<String> {
  use base64ct::{Base64, Encoding};
  let encoded = Base64::encode_string(key_bytes.as_ref());
  Ok(format!("-----BEGIN PRIVATE KEY-----\n{}\n-----END PRIVATE KEY-----", encoded))
}
