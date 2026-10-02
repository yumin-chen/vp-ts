use std::sync::Arc;
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(object)]
#[derive(Default, Clone)]
pub struct Options {
    pub home_dir: Option<String>,
}

#[napi(object)]
#[derive(Default, Clone)]
pub struct ContainerOptions {
    pub image: Option<String>,
    pub memory_mib: Option<i32>,
    pub cpus: Option<i32>,
}

#[napi(object)]
#[derive(Default, Clone)]
pub struct ContainerRestOptions {
    pub endpoint: Option<String>,
}

#[napi(object)]
#[derive(Default, Clone)]
pub struct ContainerInfo {
    pub id: String,
    pub name: Option<String>,
    pub status: String,
}

#[napi(object)]
#[derive(Default, Clone)]
pub struct RuntimeMetrics {
    pub containers_created_total: i64,
    pub num_running_containers: i64,
}

#[napi]
pub struct ImageHandle {}

#[napi]
impl ImageHandle {
    #[napi]
    pub fn list(&self) -> Vec<String> {
        vec![]
    }
}

#[napi]
pub struct VolumeHandle {}

#[napi]
impl VolumeHandle {
    #[napi]
    pub fn list(&self) -> Vec<String> {
        vec![]
    }
}

/// ContainerLite runtime instance.
#[napi]
pub struct Container {
    id: String,
    home_dir: String,
}

#[napi]
impl Container {
    #[napi(constructor)]
    pub fn new(options: Option<Options>) -> Result<Self> {
        let home_dir = options
            .and_then(|o| o.home_dir)
            .unwrap_or_else(|| "~/.container".to_string());
        Ok(Self {
            id: "default".to_string(),
            home_dir,
        })
    }

    #[napi(factory)]
    pub fn with_default_config() -> Result<Self> {
        Self::new(None)
    }

    #[napi]
    pub fn init_default(_options: Option<Options>) -> Result<()> {
        Ok(())
    }

    #[napi(factory)]
    pub fn rest(options: ContainerRestOptions) -> Result<Self> {
        Ok(Self {
            id: "rest".to_string(),
            home_dir: options.endpoint.unwrap_or_default(),
        })
    }

    #[napi(js_name = "importContainer")]
    pub fn import_container(
        &self,
        archive_path: String,
        name: Option<String>,
    ) -> Result<Container> {
        let container_name = name.unwrap_or_else(|| archive_path.clone());
        Ok(Container {
            id: container_name,
            home_dir: self.home_dir.clone(),
        })
    }

    #[napi]
    pub fn create(
        &self,
        options: Option<ContainerOptions>,
        name: Option<String>,
    ) -> Result<Container> {
        let container_name = name
            .or_else(|| options.and_then(|o| o.image))
            .unwrap_or_else(|| "container".to_string());
        Ok(Container {
            id: container_name,
            home_dir: self.home_dir.clone(),
        })
    }

    #[napi]
    pub fn get_or_create(
        &self,
        options: Option<ContainerOptions>,
        name: Option<String>,
    ) -> Result<GetOrCreateResult> {
        let container = self.create(options, name)?;
        Ok(GetOrCreateResult {
            inner_handle: Arc::new(container),
            inner_created: true,
        })
    }

    #[napi]
    pub fn list_info(&self) -> Result<Vec<ContainerInfo>> {
        Ok(vec![ContainerInfo {
            id: self.id.clone(),
            name: Some(self.id.clone()),
            status: "running".to_string(),
        }])
    }

    #[napi]
    pub fn get_info(&self, id_or_name: String) -> Result<Option<ContainerInfo>> {
        Ok(Some(ContainerInfo {
            id: id_or_name.clone(),
            name: Some(id_or_name),
            status: "running".to_string(),
        }))
    }

    #[napi]
    pub fn get(&self, id_or_name: String) -> Result<Option<Container>> {
        Ok(Some(Container {
            id: id_or_name,
            home_dir: self.home_dir.clone(),
        }))
    }

    #[napi]
    pub fn metrics(&self) -> Result<RuntimeMetrics> {
        Ok(RuntimeMetrics {
            containers_created_total: 1,
            num_running_containers: 1,
        })
    }

    #[napi(getter)]
    pub fn images(&self) -> Result<ImageHandle> {
        Ok(ImageHandle {})
    }

    #[napi(getter)]
    pub fn volumes(&self) -> Result<VolumeHandle> {
        Ok(VolumeHandle {})
    }

    #[napi]
    pub fn remove(&self, _id_or_name: String, _force: Option<bool>) -> Result<()> {
        Ok(())
    }

    #[napi]
    pub fn close(&self) -> Result<()> {
        Ok(())
    }

    #[napi]
    pub fn shutdown(&self, _timeout: Option<i32>) -> Result<()> {
        Ok(())
    }

    #[napi]
    pub fn exec(&self, _command: String, _args: Option<Vec<String>>) -> Result<String> {
        Ok("ok".to_string())
    }
}

/// Result of a `getOrCreate` operation.
#[napi]
pub struct GetOrCreateResult {
    inner_handle: Arc<Container>,
    inner_created: bool,
}

#[napi]
impl GetOrCreateResult {
    #[napi(getter)]
    pub fn created(&self) -> bool {
        self.inner_created
    }

    #[napi(getter, js_name = "container")]
    pub fn get_container(&self) -> Container {
        Container {
            id: self.inner_handle.id.clone(),
            home_dir: self.inner_handle.home_dir.clone(),
        }
    }
}
