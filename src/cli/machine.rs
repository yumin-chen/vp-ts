use napi_derive::napi;

#[napi(object)]
#[derive(Debug, Clone, Default)]
pub struct MachineCreateOptions {
    pub image: String,
    pub name: Option<String>,
    pub cpus: Option<i32>,
    pub memory: Option<String>,
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct MachineActionResult {
    pub success: bool,
    pub name: String,
}

#[napi]
pub fn create_machine_cli(options: MachineCreateOptions) -> napi::Result<MachineActionResult> {
    let name = options.name.unwrap_or_else(|| "default-machine".to_string());
    Ok(MachineActionResult {
        success: true,
        name,
    })
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct MachineListItem {
    pub name: String,
    pub state: String,
    pub is_default: bool,
}

#[napi]
pub fn list_machines_cli() -> napi::Result<Vec<MachineListItem>> {
    Ok(vec![MachineListItem {
        name: "default-machine".to_string(),
        state: "running".to_string(),
        is_default: true,
    }])
}
