use napi_derive::napi;

#[napi(object)]
#[derive(Debug, Clone)]
pub struct SystemStatusResult {
    pub running: bool,
    pub api_version: String,
}

#[napi]
pub fn system_status_cli() -> napi::Result<SystemStatusResult> {
    Ok(SystemStatusResult {
        running: true,
        api_version: "1.2.3".to_string(),
    })
}

#[napi]
pub fn system_start_cli() -> napi::Result<bool> {
    Ok(true)
}

#[napi]
pub fn system_stop_cli() -> napi::Result<bool> {
    Ok(true)
}
