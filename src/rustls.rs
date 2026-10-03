use std::sync::Arc;
use napi_derive::napi;
use rustls::crypto::CryptoProvider;

fn fetch_provider(name: &str) -> napi::Result<(String, Arc<CryptoProvider>)> {
  let normalized = name.to_lowercase();
  match normalized.as_str() {
    "ring" => Ok(("ring".to_string(), Arc::new(rustls::crypto::ring::default_provider()))),
    "openssl" => {
      let prov = rustls_openssl::custom_provider(
        rustls_openssl::ALL_CIPHER_SUITES.to_vec(),
        vec![rustls::crypto::ring::kx_group::SECP256R1],
      );
      Ok(("openssl".to_string(), Arc::new(prov)))
    }
    "btls" | "boringssl" => Ok(("btls".to_string(), Arc::new(boring_rustls_provider::provider()))),
    "mbedtls" => Ok(("mbedtls".to_string(), Arc::new(rustls_mbedcrypto_provider::mbedtls_crypto_provider()))),
    _ => Err(napi::Error::from_reason(format!("Unknown TLS provider: {}", name))),
  }
}

#[napi(js_name = "TLS")]
pub struct TLS {
  provider_name: String,
  provider: Arc<CryptoProvider>,
}

#[napi]
impl TLS {
  #[napi(constructor)]
  pub fn new(provider: Option<String>) -> napi::Result<Self> {
    let pname = provider.unwrap_or_else(|| "ring".to_string());
    let (name, prov) = fetch_provider(&pname)?;
    Ok(TLS {
      provider_name: name,
      provider: prov,
    })
  }

  #[napi(getter)]
  pub fn provider_name(&self) -> String {
    self.provider_name.clone()
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
