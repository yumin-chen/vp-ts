use napi_derive::napi;

#[napi(object)]
#[derive(Debug, Clone, Default)]
pub struct NetworkCreateOptions {
    pub name: String,
    pub internal: Option<bool>,
    pub subnet: Option<String>,
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct NetworkActionResult {
    pub success: bool,
    pub name: String,
}

#[napi]
pub fn create_network_cli(options: NetworkCreateOptions) -> napi::Result<NetworkActionResult> {
    Ok(NetworkActionResult {
        success: true,
        name: options.name,
    })
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct NetworkListItem {
    pub name: String,
    pub plugin: String,
    pub internal: bool,
}

#[napi]
pub fn list_networks_cli() -> napi::Result<Vec<NetworkListItem>> {
    Ok(vec![NetworkListItem {
        name: "default".to_string(),
        plugin: "container-network-vmnet".to_string(),
        internal: false,
    }])
}
