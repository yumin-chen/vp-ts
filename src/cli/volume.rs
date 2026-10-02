use napi_derive::napi;

#[napi(object)]
#[derive(Debug, Clone, Default)]
pub struct VolumeCreateOptions {
    pub name: String,
    pub size: Option<String>,
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct VolumeActionResult {
    pub success: bool,
    pub name: String,
}

#[napi]
pub fn create_volume_cli(options: VolumeCreateOptions) -> napi::Result<VolumeActionResult> {
    Ok(VolumeActionResult {
        success: true,
        name: options.name,
    })
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct VolumeListItem {
    pub name: String,
    pub driver: String,
}

#[napi]
pub fn list_volumes_cli() -> napi::Result<Vec<VolumeListItem>> {
    Ok(vec![VolumeListItem {
        name: "data-vol".to_string(),
        driver: "local".to_string(),
    }])
}
