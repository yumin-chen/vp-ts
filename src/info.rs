use napi_derive::napi;

#[napi(object)]
#[derive(Clone, Default)]
pub struct JsContainerState {
    pub status: String,
}

#[napi(object)]
#[derive(Clone, Default)]
pub struct JsContainerInfo {
    pub id: String,
    pub name: Option<String>,
    pub state: JsContainerState,
    pub image: String,
}

impl From<crate::container::ContainerInfoCore> for JsContainerInfo {
    fn from(core: crate::container::ContainerInfoCore) -> Self {
        JsContainerInfo {
            id: core.id,
            name: core.name,
            state: JsContainerState { status: core.status },
            image: core.image,
        }
    }
}
