use napi_derive::napi;

#[napi]
pub enum TlsProvider {
  Ring,
  Openssl,
  Btls,
  Mbedtls,
}

#[napi(js_name = "TLS")]
pub struct Tls {
  provider: TlsProvider,
}

#[napi]
impl Tls {
  #[napi(constructor)]
  pub fn new(provider: Option<TlsProvider>) -> Self {
    Self {
      provider: provider.unwrap_or(TlsProvider::Ring),
    }
  }

  #[napi]
  pub fn get_provider_name(&self) -> String {
    match self.provider {
      TlsProvider::Ring => "ring".to_string(),
      TlsProvider::Openssl => "openssl".to_string(),
      TlsProvider::Btls => "boringssl".to_string(),
      TlsProvider::Mbedtls => "mbedtls".to_string(),
    }
  }

  #[napi]
  pub fn is_supported(&self) -> bool {
    match self.provider {
      TlsProvider::Ring => true,
      TlsProvider::Openssl => true,
      TlsProvider::Btls => true,
      TlsProvider::Mbedtls => false, // mbedtls provider crate not published/available
    }
  }
}
