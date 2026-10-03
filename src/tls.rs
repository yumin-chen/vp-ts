use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
#[derive(Clone)]
pub struct Tls {
  pub provider_name: String,
}

#[napi]
impl Tls {
  #[napi(constructor)]
  pub fn new(provider: Option<String>) -> Result<Self> {
    let p_str = provider.unwrap_or_else(|| "ring".to_string()).to_lowercase();
    match p_str.as_str() {
      "ring" | "default" => Ok(Self {
        provider_name: "ring".to_string(),
      }),
      "openssl" => Ok(Self {
        provider_name: "openssl".to_string(),
      }),
      "btls" | "boringssl" => Ok(Self {
        provider_name: "btls".to_string(),
      }),
      "mbedtls" => Ok(Self {
        provider_name: "mbedtls".to_string(),
      }),
      other => Err(Error::new(
        Status::InvalidArg,
        format!("Unknown TLS provider: {}", other),
      )),
    }
  }

  #[napi]
  pub fn get_provider_name(&self) -> String {
    self.provider_name.clone()
  }

  #[napi]
  pub fn is_supported(&self) -> bool {
    true
  }
}

#[napi]
pub fn create_tls(provider: Option<String>) -> Result<Tls> {
  Tls::new(provider)
}
