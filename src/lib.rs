pub mod container;
pub mod container_handle;
pub mod images;
pub mod info;
pub mod metrics;
pub mod options;
pub mod util;
pub mod volumes;

use std::sync::Arc;

use container::{ContainerArchive, ContainerOptions, ContainerRuntime};
use napi::bindgen_prelude::*;
use napi_derive::napi;

use crate::container_handle::JsContainerHandle;
use crate::images::JsImageHandle;
use crate::info::JsContainerInfo;
use crate::metrics::JsRuntimeMetrics;
use crate::options::{JsContainerOptions, JsContainerRestOptions, JsOptions, js_options_into_core};
use crate::util::map_err;
use crate::volumes::JsVolumeHandle;

/// ContainerLite runtime instance.
///
/// The main entry point for creating and managing containers. Each runtime
/// instance manages a separate data directory with its own containers, images,
/// and configuration.
#[napi]
pub struct JsContainer {
    runtime: Arc<ContainerRuntime>,
}

#[napi]
impl JsContainer {
    /// Create a new runtime with custom options.
    #[napi(constructor)]
    pub fn new(options: JsOptions) -> Result<Self> {
        let core_opts = js_options_into_core(options)?;
        let _ = container::init_logging_for(&core_opts.home_dir);
        let runtime = ContainerRuntime::new(core_opts).map_err(map_err)?;

        Ok(Self {
            runtime: Arc::new(runtime),
        })
    }

    /// Get the default runtime instance.
    #[napi(factory)]
    pub fn with_default_config() -> Result<Self> {
        let _ = container::init_logging_for(&ContainerOptions::default().home_dir);
        let runtime = ContainerRuntime::default_runtime();
        Ok(Self {
            runtime: runtime.clone(),
        })
    }

    /// Initialize the default runtime with custom options.
    #[napi]
    pub fn init_default(options: JsOptions) -> Result<()> {
        let core_opts = js_options_into_core(options)?;
        let _ = container::init_logging_for(&core_opts.home_dir);
        ContainerRuntime::init_default_runtime(core_opts).map_err(map_err)
    }

    /// Takes the `ContainerRestOptions` class; the positional->bag
    /// adaptation lives in the binding (`JsContainerRestOptions::into_core`).
    #[napi(factory)]
    pub fn rest(options: &JsContainerRestOptions) -> Result<Self> {
        let runtime = ContainerRuntime::rest(options.into()).map_err(map_err)?;
        Ok(Self {
            runtime: Arc::new(runtime),
        })
    }

    /// Import a container from a `.container` archive.
    #[napi(js_name = "importContainer")]
    pub async fn import_container(&self, archive_path: String, name: Option<String>) -> Result<JsContainerHandle> {
        let runtime = Arc::clone(&self.runtime);
        let archive = ContainerArchive::new(archive_path);
        let handle = runtime.import_container(archive, name).await.map_err(map_err)?;
        Ok(JsContainerHandle {
            handle: Arc::new(handle),
        })
    }

    /// Create a new container.
    #[napi]
    pub async fn create(&self, options: JsContainerOptions, name: Option<String>) -> Result<JsContainerHandle> {
        let runtime = Arc::clone(&self.runtime);
        let options = ContainerOptions::try_from(options).map_err(map_err)?;
        let handle = runtime.create(options, name).await.map_err(map_err)?;

        Ok(JsContainerHandle {
            handle: Arc::new(handle),
        })
    }

    /// Get an existing container by name, or create a new one if it doesn't exist.
    #[napi]
    pub async fn get_or_create(
        &self,
        options: JsContainerOptions,
        name: Option<String>,
    ) -> Result<JsGetOrCreateResult> {
        let runtime = Arc::clone(&self.runtime);
        let options = ContainerOptions::try_from(options).map_err(map_err)?;
        let (handle, created) = runtime
            .get_or_create(options, name)
            .await
            .map_err(map_err)?;

        Ok(JsGetOrCreateResult {
            inner_handle: Arc::new(handle),
            inner_created: created,
        })
    }

    /// List all containers managed by this runtime.
    #[napi]
    pub async fn list_info(&self) -> Result<Vec<JsContainerInfo>> {
        let runtime = Arc::clone(&self.runtime);
        let infos = runtime.list_info().await.map_err(map_err)?;

        Ok(infos.into_iter().map(JsContainerInfo::from).collect())
    }

    /// Get information about a specific container by ID or name.
    #[napi]
    pub async fn get_info(&self, id_or_name: String) -> Result<Option<JsContainerInfo>> {
        let runtime = Arc::clone(&self.runtime);
        Ok(runtime
            .get_info(&id_or_name)
            .await
            .map_err(map_err)?
            .map(JsContainerInfo::from))
    }

    /// Get a container handle by ID or name (for reattach or restart).
    #[napi]
    pub async fn get(&self, id_or_name: String) -> Result<Option<JsContainerHandle>> {
        let runtime = Arc::clone(&self.runtime);
        let result = runtime.get(&id_or_name).await.map_err(map_err)?;

        let js_container = result.map(|handle| {
            JsContainerHandle {
                handle: Arc::new(handle),
            }
        });

        Ok(js_container)
    }

    /// Get runtime metrics.
    #[napi]
    pub async fn metrics(&self) -> Result<JsRuntimeMetrics> {
        let runtime = Arc::clone(&self.runtime);
        let metrics = runtime.metrics().await.map_err(map_err)?;
        Ok(JsRuntimeMetrics::from(metrics))
    }

    /// Get the runtime image handle.
    #[napi(getter)]
    pub fn images(&self) -> Result<JsImageHandle> {
        let handle = self.runtime.images().map_err(map_err)?;
        Ok(JsImageHandle {
            handle: Arc::new(handle),
        })
    }

    /// Get the runtime volume handle.
    #[napi(getter)]
    pub fn volumes(&self) -> Result<JsVolumeHandle> {
        let handle = self.runtime.volumes().map_err(map_err)?;
        Ok(JsVolumeHandle {
            handle: Arc::new(handle),
        })
    }

    /// Remove a container by ID or name.
    #[napi]
    pub async fn remove(&self, id_or_name: String, force: Option<bool>) -> Result<()> {
        let runtime = Arc::clone(&self.runtime);
        runtime
            .remove(&id_or_name, force.unwrap_or(false))
            .await
            .map_err(map_err)
    }

    /// Close the runtime (no-op, provided for API compatibility).
    #[napi]
    pub fn close(&self) -> Result<()> {
        Ok(())
    }

    /// Gracefully shutdown all containers in this runtime.
    #[napi]
    pub async fn shutdown(&self, timeout: Option<i32>) -> Result<()> {
        let runtime = Arc::clone(&self.runtime);
        runtime.shutdown(timeout).await.map_err(map_err)
    }
}

/// Result of a `getOrCreate` operation.
#[napi]
pub struct JsGetOrCreateResult {
    inner_handle: Arc<container::LiteContainer>,
    inner_created: bool,
}

#[napi]
impl JsGetOrCreateResult {
    /// Whether the container was newly created (true) or already existed (false).
    #[napi(getter)]
    pub fn created(&self) -> bool {
        self.inner_created
    }

    /// The container handle.
    #[napi(getter, js_name = "container")]
    pub fn get_container(&self) -> JsContainerHandle {
        JsContainerHandle {
            handle: Arc::clone(&self.inner_handle),
        }
    }
}
