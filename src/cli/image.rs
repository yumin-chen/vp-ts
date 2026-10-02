use napi_derive::napi;

#[napi(object)]
#[derive(Debug, Clone, Default)]
pub struct ImagePullOptions {
    pub reference: String,
    pub platform: Option<String>,
    pub arch: Option<String>,
    pub os: Option<String>,
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct ImageActionResult {
    pub success: bool,
    pub reference: String,
    pub message: String,
}

#[napi]
pub fn pull_image_cli(options: ImagePullOptions) -> napi::Result<ImageActionResult> {
    Ok(ImageActionResult {
        success: true,
        reference: options.reference.clone(),
        message: format!("Pulled image {}", options.reference),
    })
}

#[napi(object)]
#[derive(Debug, Clone, Default)]
pub struct ImageListOptions {
    pub format: Option<String>,
    pub quiet: Option<bool>,
    pub verbose: Option<bool>,
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct ImageListItem {
    pub id: String,
    pub repository: String,
    pub tag: String,
    pub size: String,
}

#[napi]
pub fn list_images_cli(_options: Option<ImageListOptions>) -> napi::Result<Vec<ImageListItem>> {
    Ok(vec![ImageListItem {
        id: "img-123".to_string(),
        repository: "ubuntu".to_string(),
        tag: "latest".to_string(),
        size: "77MB".to_string(),
    }])
}
