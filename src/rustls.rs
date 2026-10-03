use std::sync::Arc;
use napi_derive::napi;
use rustls::crypto::CryptoProvider;

fn fetch_provider(name: &str) -> (String, Arc<CryptoProvider>, bool) {
  let normalized = name.to_lowercase();
  match normalized.as_str() {
    "openssl" => {
      let prov = rustls_openssl::custom_provider(
        rustls_openssl::ALL_CIPHER_SUITES.to_vec(),
        vec![rustls::crypto::ring::kx_group::SECP256R1],
      );
      ("openssl".to_string(), Arc::new(prov), false)
    }
    "btls" | "boringssl" => {
      let prov = boring_rustls_provider::provider();
      ("btls".to_string(), Arc::new(prov), false)
    }
    "mbedtls" => {
      let prov = rustls_mbedcrypto_provider::mbedtls_crypto_provider();
      ("mbedtls".to_string(), Arc::new(prov), false)
    }
    _ => {
      // Primary default: ring
      let prov = rustls::crypto::ring::default_provider();
      ("ring".to_string(), Arc::new(prov), normalized != "ring")
    }
  }
}

#[napi(js_name = "TLS")]
pub struct TLS {
  provider_name: String,
  provider: Arc<CryptoProvider>,
  fallback_used: bool,
}

#[napi]
impl TLS {
  #[napi(constructor)]
  pub fn new(provider: Option<String>) -> Self {
    let pname = provider.unwrap_or_else(|| "ring".to_string());
    let (name, prov, fallback) = fetch_provider(&pname);
    TLS {
      provider_name: name,
      provider: prov,
      fallback_used: fallback,
    }
  }

  #[napi(getter)]
  pub fn provider_name(&self) -> String {
    self.provider_name.clone()
  }

  #[napi(getter)]
  pub fn is_fallback(&self) -> bool {
    self.fallback_used
  }

  #[napi]
  pub fn get_provider_name(&self) -> String {
    self.provider_name.clone()
  }

  #[napi]
  pub fn get_available_providers() -> Vec<String> {
    vec!["ring".into(), "openssl".into(), "btls".into(), "mbedtls".into()]
  }

  #[napi]
  pub fn get_cipher_suites(&self) -> Vec<String> {
    self
      .provider
      .cipher_suites
      .iter()
      .map(|cs| format!("{:?}", cs.suite()))
      .collect()
  }

  #[napi]
  pub fn is_cipher_supported(&self, cipher: String) -> bool {
    let lower = cipher.to_lowercase();
    self.get_cipher_suites().iter().any(|c| c.to_lowercase().contains(&lower))
  }
}
