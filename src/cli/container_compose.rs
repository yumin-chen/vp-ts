use napi_derive::napi;

#[napi(object)]
#[derive(Default, Clone)]
pub struct ComposeStatusOptions {
    pub socket: Option<String>,
    pub address: Option<String>,
    pub cacert: Option<String>,
}

#[napi(object)]
#[derive(Default, Clone)]
pub struct GenerateKeyOptions {
    pub name: Option<String>,
    pub auth_file: Option<String>,
}

#[napi(object)]
#[derive(Default, Clone)]
pub struct GenerateCertOptions {
    pub out_dir: Option<String>,
    pub cn: Option<String>,
    pub days: Option<i32>,
    pub san_dns: Option<Vec<String>>,
    pub san_ip: Option<Vec<String>>,
    pub force: Option<bool>,
}

#[napi(object)]
#[derive(Default, Clone)]
pub struct ListKeysOptions {
    pub auth_file: Option<String>,
}

#[napi(object)]
#[derive(Default, Clone)]
pub struct RevokeKeyOptions {
    pub name: String,
    pub auth_file: Option<String>,
}

#[napi]
pub struct ContainerCompose {}

#[napi]
impl ContainerCompose {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {}
    }

    #[napi]
    pub fn status(&self, _options: Option<ComposeStatusOptions>) -> bool {
        true
    }

    #[napi]
    pub fn generate_key(&self, options: Option<GenerateKeyOptions>) -> String {
        let name = options.and_then(|o| o.name).unwrap_or_else(|| "default".to_string());
        format!("key-{}", name)
    }

    #[napi]
    pub fn generate_cert(&self, _options: Option<GenerateCertOptions>) -> bool {
        true
    }

    #[napi]
    pub fn list_keys(&self, _options: Option<ListKeysOptions>) -> Vec<String> {
        vec![]
    }

    #[napi]
    pub fn revoke_key(&self, options: RevokeKeyOptions) -> bool {
        !options.name.is_empty()
    }
}
