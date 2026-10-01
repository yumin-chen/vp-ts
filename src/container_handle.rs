use napi::Result;
use napi_derive::napi;
use std::sync::Arc;

#[napi]
pub struct JsContainerHandle {
    #[allow(dead_code)]
    pub(crate) handle: Arc<crate::container::LiteContainer>,
}

#[napi]
impl JsContainerHandle {
    #[napi]
    pub async fn exec(&self, command: String, args: Option<Vec<String>>) -> Result<String> {
        let args_str = args.unwrap_or_default().join(" ");
        Ok(format!("Executed: {} {}", command, args_str))
    }

    #[napi]
    pub async fn stop(&self) -> Result<()> {
        Ok(())
    }

    #[napi]
    pub async fn start(&self) -> Result<()> {
        Ok(())
    }
}
