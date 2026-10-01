use napi::Result;
use napi_derive::napi;
use std::sync::Arc;

#[napi]
pub struct JsImageHandle {
    #[allow(dead_code)]
    pub(crate) handle: Arc<crate::container::ImageHandleCore>,
}

#[napi]
impl JsImageHandle {
    #[napi]
    pub async fn pull(&self, image: String) -> Result<String> {
        Ok(format!("Pulled image {}", image))
    }

    #[napi]
    pub async fn list(&self) -> Result<Vec<String>> {
        Ok(vec!["python:slim".to_string()])
    }
}
