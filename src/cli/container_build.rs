use napi_derive::napi;

#[napi(object)]
#[derive(Default, Clone)]
pub struct BuildOptions {
    pub context_dir: Option<String>,
    pub file: Option<String>,
    pub target: Option<String>,
    pub tag: Option<Vec<String>>,
    pub cpus: Option<i32>,
    pub memory: Option<String>,
    pub no_cache: Option<bool>,
    pub pull: Option<bool>,
    pub secret: Option<Vec<String>>,
    pub ssh: Option<String>,
    pub vsock_port: Option<i32>,
}

#[napi(object)]
#[derive(Default, Clone)]
pub struct BuilderStartOptions {
    pub cpus: Option<i32>,
    pub memory: Option<String>,
    pub ssh: Option<String>,
    pub dns: Option<Vec<String>>,
}

#[napi(object)]
#[derive(Default, Clone)]
pub struct BuilderStatus {
    pub running: bool,
    pub cpus: i32,
    pub memory: String,
}

#[napi]
pub struct ContainerBuild {}

#[napi]
impl ContainerBuild {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {}
    }

    #[napi]
    pub fn build(&self, options: Option<BuildOptions>) -> String {
        let tag = options
            .and_then(|o| o.tag)
            .and_then(|t| t.first().cloned())
            .unwrap_or_else(|| "latest".to_string());
        format!("Successfully built {}", tag)
    }

    #[napi]
    pub fn builder_start(&self, _options: Option<BuilderStartOptions>) -> bool {
        true
    }

    #[napi]
    pub fn builder_status(&self) -> BuilderStatus {
        BuilderStatus {
            running: true,
            cpus: 2,
            memory: "2048MB".to_string(),
        }
    }

    #[napi]
    pub fn builder_stop(&self) -> bool {
        true
    }

    #[napi]
    pub fn builder_delete(&self) -> bool {
        true
    }
}
