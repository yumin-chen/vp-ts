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
  fallback_used: bool,
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

    Ok(Self {
      provider_type,
      fallback_used: false,
    })
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

  #[napi]
  pub fn fallback_used(&self) -> bool {
    self.fallback_used
  }

  /// Installs or verifies the selected provider as the default rustls CryptoProvider,
  /// falling back safely to ring if the selected optional provider fails.
  #[napi]
  pub fn install_as_default(&mut self) -> Result<bool> {
    let provider_res: std::result::Result<CryptoProvider, ()> = std::panic::catch_unwind(|| {
      match self.provider_type {
        TlsProvider::Ring => rustls::crypto::ring::default_provider(),
        TlsProvider::OpenSSL => rustls_openssl::default_provider(),
        TlsProvider::BoringSSL => boring_rustls_provider::provider(),
        TlsProvider::MbedTLS => rustls_mbedtls_provider::mbedtls_crypto_provider(),
      }
    })
    .map_err(|_| ());

    let (provider, is_fallback) = match provider_res {
      Ok(p) => (p, false),
      Err(_) => {
        // Fallback to ring default provider
        (rustls::crypto::ring::default_provider(), true)
      }
    };

    self.fallback_used = is_fallback;

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
