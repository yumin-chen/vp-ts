use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub enum TlsProvider {
  Ring,
  OpenSSL,
  BoringSSL,
  MbedTLS,
}

#[napi(js_name = "TLS")]
pub struct TLS {
  provider_name: String,
}

#[napi]
impl TLS {
  #[napi(constructor)]
  pub fn new(provider: Option<String>) -> Result<Self> {
    let prov = provider.unwrap_or_else(|| "ring".to_string()).to_lowercase();
    match prov.as_str() {
      "ring" | "default" => Ok(TLS {
        provider_name: "ring".to_string(),
      }),
      "openssl" | "rustls-openssl" => Ok(TLS {
        provider_name: "openssl".to_string(),
      }),
      "boringssl" | "btls" | "boring-rustls-provider" => Ok(TLS {
        provider_name: "boringssl".to_string(),
      }),
      "mbedtls" | "rustls-mbedtls-provider" => Ok(TLS {
        provider_name: "mbedtls".to_string(),
      }),
      _ => Err(Error::from_reason(format!("Unknown TLS provider: {prov}"))),
    }
  }

  #[napi]
  pub fn get_provider(&self) -> String {
    self.provider_name.clone()
  }

  #[napi]
  pub fn is_supported(&self) -> bool {
    match self.provider_name.as_str() {
      "ring" => true,
      "openssl" | "boringssl" | "mbedtls" => true,
      _ => false,
    }
  }

  #[napi]
  pub fn connect(&self, host: String, port: u16) -> Result<String> {
    Ok(format!(
      "TLS connection initiated to {}:{} using {} provider",
      host, port, self.provider_name
    ))
  }
}
