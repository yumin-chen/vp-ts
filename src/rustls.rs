use napi_derive::napi;
use std::sync::Arc;

use rustls::crypto::CryptoProvider;

pub fn _get_ring_provider() -> Arc<CryptoProvider> {
  Arc::new(rustls::crypto::ring::default_provider())
}

pub fn _get_openssl_provider() -> Arc<CryptoProvider> {
  Arc::new(rustls_openssl::default_provider())
}

pub fn _get_boring_provider() -> Arc<CryptoProvider> {
  Arc::new(boring_rustls_provider::provider())
}

pub fn _get_mbedtls_provider_supported() -> bool {
  true
}

#[napi]
pub fn get_default_provider_name() -> String {
  "ring".to_string()
}

#[napi]
pub fn get_supported_providers() -> Vec<String> {
  vec![
    "ring".to_string(),
    "openssl".to_string(),
    "btls".to_string(),
    "mbedtls".to_string(),
  ]
}
