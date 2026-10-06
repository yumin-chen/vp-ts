use napi_derive::napi;

#[napi]
pub enum TlsProvider {
  Ring,
  OpenSSL,
  BoringSSL,
  MbedTLS,
}

#[napi]
pub struct TLS {
  provider: TlsProvider,
  alpn_protocols: Vec<String>,
}

#[napi]
impl TLS {
  #[napi(constructor)]
  pub fn new(provider: Option<TlsProvider>) -> Self {
    Self {
      provider: provider.unwrap_or(TlsProvider::Ring),
      alpn_protocols: Vec::new(),
    }
  }

  #[napi]
  pub fn set_alpn_protocols(&mut self, protocols: Vec<String>) {
    self.alpn_protocols = protocols;
  }

  #[napi]
  pub fn get_provider_name(&self) -> String {
    match self.provider {
      TlsProvider::Ring => "ring".to_string(),
      TlsProvider::OpenSSL => "rustls-openssl".to_string(),
      TlsProvider::BoringSSL => "boring-rustls-provider".to_string(),
      TlsProvider::MbedTLS => "rustls-mbedtls-provider".to_string(),
    }
  }

  #[napi]
  pub fn is_supported(&self) -> bool {
    matches!(self.provider, TlsProvider::Ring)
  }
}
