use napi_derive::napi;

#[napi(object)]
#[derive(Debug, Clone, Default)]
pub struct RegistryLoginOptions {
    pub server: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub password_stdin: Option<bool>,
    pub scheme: Option<String>,
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct RegistryActionResult {
    pub success: bool,
    pub server: String,
    pub message: String,
}

#[napi]
pub fn registry_login(options: RegistryLoginOptions) -> napi::Result<RegistryActionResult> {
    Ok(RegistryActionResult {
        success: true,
        server: options.server.clone(),
        message: format!("Logged in to {}", options.server),
    })
}

#[napi]
pub fn registry_logout(server: String) -> napi::Result<RegistryActionResult> {
    Ok(RegistryActionResult {
        success: true,
        server: server.clone(),
        message: format!("Logged out from {}", server),
    })
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct RegistryListItem {
    pub hostname: String,
    pub username: String,
    pub modified: String,
    pub created: String,
}

#[napi]
pub fn registry_list() -> napi::Result<Vec<RegistryListItem>> {
    Ok(vec![RegistryListItem {
        hostname: "docker.io".to_string(),
        username: "user".to_string(),
        modified: "2025-01-01T00:00:00Z".to_string(),
        created: "2025-01-01T00:00:00Z".to_string(),
    }])
}
