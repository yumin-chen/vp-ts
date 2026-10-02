use napi_derive::napi;

#[napi(object)]
#[derive(Default, Clone)]
pub struct RegistryLoginOptions {
    pub server: String,
    pub username: Option<String>,
    pub password_stdin: Option<bool>,
    pub scheme: Option<String>,
}

#[napi(object)]
#[derive(Default, Clone)]
pub struct RegistryLogoutOptions {
    pub registry: String,
}

#[napi(object)]
#[derive(Default, Clone)]
pub struct RegistryListOptions {
    pub format: Option<String>,
    pub quiet: Option<bool>,
}

#[napi(object)]
#[derive(Default, Clone)]
pub struct RegistryResource {
    pub hostname: String,
    pub username: String,
}

#[napi]
pub struct ContainerRegistry {}

#[napi]
impl ContainerRegistry {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {}
    }

    #[napi]
    pub fn login(&self, options: RegistryLoginOptions) -> bool {
        !options.server.is_empty()
    }

    #[napi]
    pub fn logout(&self, options: RegistryLogoutOptions) -> bool {
        !options.registry.is_empty()
    }

    #[napi]
    pub fn list(&self, _options: Option<RegistryListOptions>) -> Vec<RegistryResource> {
        vec![]
    }
}
