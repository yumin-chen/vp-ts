use napi_derive::napi;

#[napi(object)]
#[derive(Clone, Default)]
pub struct JsRuntimeMetrics {
    pub containers_created_total: i64,
    pub num_running_containers: i64,
}

impl From<crate::container::RuntimeMetricsCore> for JsRuntimeMetrics {
    fn from(core: crate::container::RuntimeMetricsCore) -> Self {
        JsRuntimeMetrics {
            containers_created_total: core.containers_created_total,
            num_running_containers: core.num_running_containers,
        }
    }
}
