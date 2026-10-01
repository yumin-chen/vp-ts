use crate::container::ContainerOptions;
use napi_derive::napi;

#[napi(object)]
#[derive(Default, Clone)]
pub struct JsOptions {
    pub home_dir: Option<String>,
}

#[napi(object)]
#[derive(Default, Clone)]
pub struct JsContainerOptions {
    pub image: String,
    pub memory_mib: Option<i32>,
    pub cpus: Option<i32>,
    pub volumes: Option<Vec<String>>,
    pub env: Option<Vec<String>>,
    pub cmd: Option<Vec<String>>,
}

#[napi]
#[derive(Default, Clone)]
pub struct JsContainerRestOptions {
    pub endpoint: Option<String>,
    pub token: Option<String>,
}

#[napi]
impl JsContainerRestOptions {
    #[napi(constructor)]
    pub fn new(endpoint: Option<String>, token: Option<String>) -> Self {
        Self { endpoint, token }
    }
}

pub fn js_options_into_core(options: JsOptions) -> Result<ContainerOptions, napi::Error> {
    let mut core = ContainerOptions::default();
    if let Some(home) = options.home_dir {
        core.home_dir = home;
    }
    Ok(core)
}

impl From<&JsContainerRestOptions> for ContainerOptions {
    fn from(opts: &JsContainerRestOptions) -> Self {
        let mut core = ContainerOptions::default();
        if let Some(ref ep) = opts.endpoint {
            core.home_dir = ep.clone();
        }
        core
    }
}
