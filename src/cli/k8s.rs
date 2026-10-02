use napi_derive::napi;

#[napi(object)]
#[derive(Debug, Clone, Default)]
pub struct K8sCreateOptions {
    pub name: Option<String>,
    pub node_image: Option<String>,
    pub cpus: Option<i32>,
    pub memory: Option<String>,
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct K8sActionResult {
    pub success: bool,
    pub cluster_name: String,
}

#[napi(js_name = "k8sCreateCli")]
pub fn k8s_create_cli(options: Option<K8sCreateOptions>) -> napi::Result<K8sActionResult> {
    let opts = options.unwrap_or_default();
    let cluster_name = opts.name.unwrap_or_else(|| "k8s-dev".to_string());
    Ok(K8sActionResult {
        success: true,
        cluster_name,
    })
}

#[napi(js_name = "k8sDeleteCli")]
pub fn k8s_delete_cli(name: Option<String>) -> napi::Result<bool> {
    let _ = name;
    Ok(true)
}
