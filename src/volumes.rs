use napi::Result;
use napi_derive::napi;
use std::sync::Arc;

#[napi]
pub struct JsVolumeHandle {
    #[allow(dead_code)]
    pub(crate) handle: Arc<crate::container::VolumeHandleCore>,
}

#[napi]
impl JsVolumeHandle {
    #[napi]
    pub async fn create(&self, name: String) -> Result<String> {
        Ok(name)
    }

    #[napi]
    pub async fn list(&self) -> Result<Vec<String>> {
        Ok(vec![])
    }
}
