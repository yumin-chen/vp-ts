use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub struct TLS {
  provider: String,
  ciphersuites: Vec<String>,
  is_connected: bool,
}

#[napi]
impl TLS {
  #[napi(constructor)]
  pub fn new(provider: Option<String>) -> Self {
    TLS {
      provider: provider.unwrap_or_else(|| "ring".to_string()),
      ciphersuites: vec![
        "TLS_AES_256_GCM_SHA384".to_string(),
        "TLS_AES_128_GCM_SHA256".to_string(),
        "TLS_CHACHA20_POLY1305_SHA256".to_string(),
      ],
      is_connected: false,
    }
  }

  #[napi(getter)]
  pub fn provider(&self) -> String {
    self.provider.clone()
  }

  #[napi(getter)]
  pub fn ciphersuites(&self) -> Vec<String> {
    self.ciphersuites.clone()
  }

  #[napi]
  pub fn connect(&mut self, _host: String, _port: u16) -> Result<bool> {
    self.is_connected = true;
    Ok(true)
  }

  #[napi]
  pub fn is_connected(&self) -> bool {
    self.is_connected
  }
}
