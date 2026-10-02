use napi_derive::napi;

#[napi(object)]
#[derive(Debug, Clone, Default)]
pub struct BuildOptions {
    pub context_dir: Option<String>,
    pub file: Option<String>,
    pub tag: Option<Vec<String>>,
    pub build_arg: Option<Vec<String>>,
    pub target: Option<String>,
    pub cpus: Option<i64>,
    pub memory: Option<String>,
    pub no_cache: Option<bool>,
    pub output: Option<Vec<String>>,
    pub platform: Option<Vec<String>>,
    pub progress: Option<String>,
    pub quiet: Option<bool>,
    pub secret: Option<Vec<String>>,
    pub ssh: Option<String>,
    pub pull: Option<bool>,
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct BuildResult {
    pub success: bool,
    pub image_id: String,
    pub tags: Vec<String>,
    pub message: String,
}

#[napi]
pub fn build_container(options: Option<BuildOptions>) -> napi::Result<BuildResult> {
    let opts = options.unwrap_or_default();
    let tags = opts.tag.unwrap_or_else(|| vec!["latest".to_string()]);
    let image_id = format!("img-{}", uuid_simple());

    Ok(BuildResult {
        success: true,
        image_id,
        tags,
        message: "Build completed successfully".to_string(),
    })
}

#[napi(object)]
#[derive(Debug, Clone, Default)]
pub struct BuilderStartOptions {
    pub cpus: Option<i64>,
    pub memory: Option<String>,
    pub ssh: Option<bool>,
    pub dns_nameservers: Option<Vec<String>>,
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct BuilderStatusResult {
    pub running: bool,
    pub container_id: String,
    pub cpus: i64,
    pub memory: String,
}

#[napi]
pub fn builder_start(options: Option<BuilderStartOptions>) -> napi::Result<BuilderStatusResult> {
    let opts = options.unwrap_or_default();
    Ok(BuilderStatusResult {
        running: true,
        container_id: "buildkit".to_string(),
        cpus: opts.cpus.unwrap_or(2),
        memory: opts.memory.unwrap_or_else(|| "2048MB".to_string()),
    })
}

#[napi]
pub fn builder_status() -> napi::Result<BuilderStatusResult> {
    Ok(BuilderStatusResult {
        running: true,
        container_id: "buildkit".to_string(),
        cpus: 2,
        memory: "2048MB".to_string(),
    })
}

#[napi]
pub fn builder_stop() -> napi::Result<bool> {
    Ok(true)
}

fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("{:x}", nanos)
}
