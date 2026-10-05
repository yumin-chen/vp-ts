use napi_derive::napi;

#[napi]
pub struct Tls {
  pub provider: String,
}

#[napi]
impl Tls {
  #[napi(constructor)]
  pub fn new(provider: Option<String>) -> Self {
    Self {
      provider: provider.unwrap_or_else(|| "ring".to_string()),
    }
  }

  #[napi]
  pub fn get_provider(&self) -> String {
    self.provider.clone()
  }
}

#[napi]
pub fn create_tls(provider: Option<String>) -> Tls {
  Tls::new(provider)
}
