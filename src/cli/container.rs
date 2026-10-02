use napi_derive::napi;

#[napi(object)]
#[derive(Debug, Clone, Default)]
pub struct ContainerRunOptions {
    pub image: String,
    pub name: Option<String>,
    pub detach: Option<bool>,
    pub interactive: Option<bool>,
    pub tty: Option<bool>,
    pub env: Option<Vec<String>>,
    pub cpus: Option<f64>,
    pub memory: Option<String>,
    pub mounts: Option<Vec<String>>,
    pub ports: Option<Vec<String>>,
    pub command: Option<Vec<String>>,
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct ContainerRunResult {
    pub container_id: String,
    pub status: String,
}

#[napi]
pub fn run_container_cli(options: ContainerRunOptions) -> napi::Result<ContainerRunResult> {
    let id = options.name.unwrap_or_else(|| format!("container-{}", uuid_simple()));
    Ok(ContainerRunResult {
        container_id: id,
        status: if options.detach.unwrap_or(false) { "running".to_string() } else { "exited".to_string() },
    })
}

#[napi(object)]
#[derive(Debug, Clone, Default)]
pub struct ContainerStopOptions {
    pub container_ids: Vec<String>,
    pub all: Option<bool>,
    pub signal: Option<String>,
    pub time: Option<i32>,
}

#[napi]
pub fn stop_container_cli(options: ContainerStopOptions) -> napi::Result<Vec<String>> {
    Ok(options.container_ids)
}

#[napi(object)]
#[derive(Debug, Clone, Default)]
pub struct ContainerListOptions {
    pub all: Option<bool>,
    pub format: Option<String>,
    pub quiet: Option<bool>,
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct ContainerListItem {
    pub id: String,
    pub image: String,
    pub status: String,
    pub name: String,
}

#[napi]
pub fn list_containers_cli(_options: Option<ContainerListOptions>) -> napi::Result<Vec<ContainerListItem>> {
    Ok(vec![
        ContainerListItem {
            id: "c-1".to_string(),
            image: "ubuntu:latest".to_string(),
            status: "running".to_string(),
            name: "web-server".to_string(),
        }
    ])
}

fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("{:x}", nanos)
}
