use napi_derive::napi;

#[napi]
pub enum TlsProvider {
  Ring,
  OpenSSL,
  BoringSSL,
  MbedTLS,
}

#[napi]
pub struct Tls {
  provider: TlsProvider,
}

#[napi]
impl Tls {
  #[napi(constructor)]
  pub fn new(provider: Option<TlsProvider>) -> Self {
    Tls {
      provider: provider.unwrap_or(TlsProvider::Ring),
    }
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
    true
  }
}
