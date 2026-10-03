#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use rustls::crypto::CryptoProvider;

#[napi]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TlsProvider {
  Ring,
  OpenSSL,
  BoringSSL,
  MbedTLS,
}

#[napi]
pub struct TLS {
  provider_type: TlsProvider,
}

#[napi]
impl TLS {
  #[napi(constructor)]
  pub fn new(provider: Option<String>) -> Result<Self> {
    let provider_type = match provider.as_deref().unwrap_or("ring").to_lowercase().as_str() {
      "ring" | "default" => TlsProvider::Ring,
      "openssl" => TlsProvider::OpenSSL,
      "boringssl" | "boring" | "btls" => TlsProvider::BoringSSL,
      "mbedtls" | "mbed" => TlsProvider::MbedTLS,
      unknown => {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Unknown TLS provider: {unknown}"),
        ))
      }
    };

    Ok(Self { provider_type })
  }

  #[napi]
  pub fn provider_name(&self) -> String {
    match self.provider_type {
      TlsProvider::Ring => "ring".to_string(),
      TlsProvider::OpenSSL => "openssl".to_string(),
      TlsProvider::BoringSSL => "boringssl".to_string(),
      TlsProvider::MbedTLS => "mbedtls".to_string(),
    }
  }

  #[napi]
  pub fn get_crypto_provider(&self) -> Result<String> {
    match self.provider_type {
      TlsProvider::Ring => Ok("rustls::crypto::ring".to_string()),
      TlsProvider::OpenSSL => Ok("rustls-openssl".to_string()),
      TlsProvider::BoringSSL => Ok("boring-rustls-provider".to_string()),
      TlsProvider::MbedTLS => Ok("rustls-mbedtls-provider".to_string()),
    }
  }

  #[napi]
  pub fn is_supported(&self) -> bool {
    true
  }

  /// Installs or verifies the selected provider as the default rustls CryptoProvider
  #[napi]
  pub fn install_as_default(&self) -> Result<bool> {
    let provider: CryptoProvider = match self.provider_type {
      TlsProvider::Ring => rustls::crypto::ring::default_provider(),
      TlsProvider::OpenSSL => rustls_openssl::default_provider(),
      TlsProvider::BoringSSL => boring_rustls_provider::provider(),
      TlsProvider::MbedTLS => rustls_mbedtls_provider::mbedtls_crypto_provider(),
    };

    match provider.install_default() {
      Ok(()) => Ok(true),
      Err(_) => Ok(false), // already installed
    }
  }
}

#[napi]
pub fn create_tls(provider: Option<String>) -> Result<TLS> {
  TLS::new(provider)
}
