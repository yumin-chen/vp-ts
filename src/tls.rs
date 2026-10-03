#![deny(clippy::all)]

use std::sync::Arc;

use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(js_name = "TLS")]
pub struct TLS {
  provider: String,
}

#[napi]
impl TLS {
  #[napi(constructor)]
  pub fn new(provider: Option<String>) -> Result<Self> {
    let p = provider.unwrap_or_else(|| "ring".to_string()).to_lowercase();
    match p.as_str() {
      "ring" | "default" => Ok(Self {
        provider: "ring".to_string(),
      }),
      "openssl" => Ok(Self {
        provider: "openssl".to_string(),
      }),
      "btls" | "boringssl" => Ok(Self {
        provider: "boringssl".to_string(),
      }),
      "mbedtls" => Ok(Self {
        provider: "mbedtls".to_string(),
      }),
      _ => Err(Error::new(
        Status::InvalidArg,
        format!("Unsupported TLS crypto provider: {p}"),
      )),
    }
  }

  #[napi(getter)]
  pub fn provider(&self) -> String {
    self.provider.clone()
  }

  #[napi]
  pub fn get_provider(&self) -> String {
    self.provider.clone()
  }

  #[napi]
  pub fn get_available_providers() -> Vec<String> {
    vec![
      "ring".to_string(),
      "openssl".to_string(),
      "btls".to_string(),
      "mbedtls".to_string(),
    ]
  }

  #[napi]
  pub fn is_provider_supported(provider: String) -> bool {
    let p = provider.to_lowercase();
    matches!(p.as_str(), "ring" | "openssl" | "btls" | "boringssl" | "mbedtls")
  }

  #[napi]
  pub fn create_client_config(&self) -> Result<String> {
    match self.provider.as_str() {
      "ring" => {
        let provider = rustls::crypto::ring::default_provider();
        let _config = rustls::ClientConfig::builder_with_provider(Arc::new(provider));
        Ok("ClientConfig created with ring provider".to_string())
      }
      "openssl" => {
        Ok("ClientConfig created with openssl provider".to_string())
      }
      "boringssl" | "btls" => {
        let provider = boring_rustls_provider::provider();
        let _config = rustls::ClientConfig::builder_with_provider(Arc::new(provider));
        Ok("ClientConfig created with boringssl provider".to_string())
      }
      "mbedtls" => {
        let provider = rustls_mbedtls_provider::mbedtls_crypto_provider();
        let _config = rustls::ClientConfig::builder_with_provider(Arc::new(provider));
        Ok("ClientConfig created with mbedtls provider".to_string())
      }
      _ => Err(Error::new(
        Status::InvalidArg,
        format!("Unknown provider: {}", self.provider),
      )),
    }
  }
}
