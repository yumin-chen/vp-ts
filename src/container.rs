use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

static COUNTER: AtomicU64 = AtomicU64::new(1);
static DEFAULT_RUNTIME: Mutex<Option<Arc<ContainerRuntime>>> = Mutex::new(None);

#[derive(Clone, Debug)]
pub struct ContainerOptions {
    pub home_dir: String,
    pub image: String,
    pub memory_mib: i32,
    pub cpus: i32,
}

impl Default for ContainerOptions {
    fn default() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        ContainerOptions {
            home_dir: format!("{}/.container", home),
            image: "default".to_string(),
            memory_mib: 512,
            cpus: 1,
        }
    }
}

impl TryFrom<crate::options::JsContainerOptions> for ContainerOptions {
    type Error = String;
    fn try_from(opts: crate::options::JsContainerOptions) -> Result<Self, Self::Error> {
        Ok(ContainerOptions {
            home_dir: "/tmp/.container".to_string(),
            image: opts.image,
            memory_mib: opts.memory_mib.unwrap_or(512),
            cpus: opts.cpus.unwrap_or(1),
        })
    }
}

#[derive(Clone, Debug)]
pub struct ContainerArchive {
    pub path: String,
}

impl ContainerArchive {
    pub fn new(path: String) -> Self {
        ContainerArchive { path }
    }
}

#[derive(Clone, Debug)]
pub struct LiteContainer {
    pub id: String,
    pub name: Option<String>,
    pub image: String,
    pub status: String,
}

#[derive(Clone, Debug)]
pub struct ContainerInfoCore {
    pub id: String,
    pub name: Option<String>,
    pub status: String,
    pub image: String,
}

#[derive(Clone, Debug, Default)]
pub struct RuntimeMetricsCore {
    pub containers_created_total: i64,
    pub num_running_containers: i64,
}

#[derive(Clone, Debug, Default)]
pub struct ImageHandleCore;

#[derive(Clone, Debug, Default)]
pub struct VolumeHandleCore;

#[derive(Clone)]
pub struct ContainerRuntime {
    pub options: ContainerOptions,
    containers: Arc<Mutex<HashMap<String, LiteContainer>>>,
}

impl ContainerRuntime {
    pub fn new(options: ContainerOptions) -> Result<Self, String> {
        Ok(ContainerRuntime {
            options,
            containers: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub fn default_runtime() -> Arc<ContainerRuntime> {
        let mut guard = DEFAULT_RUNTIME.lock().unwrap();
        if let Some(ref rt) = *guard {
            rt.clone()
        } else {
            let rt = Arc::new(ContainerRuntime::new(ContainerOptions::default()).unwrap());
            *guard = Some(rt.clone());
            rt
        }
    }

    pub fn init_default_runtime(options: ContainerOptions) -> Result<(), String> {
        let mut guard = DEFAULT_RUNTIME.lock().unwrap();
        let rt = Arc::new(ContainerRuntime::new(options)?);
        *guard = Some(rt);
        Ok(())
    }

    pub fn rest(options: ContainerOptions) -> Result<Self, String> {
        Self::new(options)
    }

    pub async fn import_container(&self, archive: ContainerArchive, name: Option<String>) -> Result<LiteContainer, String> {
        let cnt = COUNTER.fetch_add(1, Ordering::Relaxed);
        let id = format!("ctr-{:x}", cnt);
        let container = LiteContainer {
            id: id.clone(),
            name,
            image: archive.path,
            status: "imported".to_string(),
        };
        self.containers.lock().unwrap().insert(id, container.clone());
        Ok(container)
    }

    pub async fn create(&self, options: ContainerOptions, name: Option<String>) -> Result<LiteContainer, String> {
        let cnt = COUNTER.fetch_add(1, Ordering::Relaxed);
        let id = format!("ctr-{:x}", cnt);
        let container = LiteContainer {
            id: id.clone(),
            name,
            image: options.image,
            status: "running".to_string(),
        };
        self.containers.lock().unwrap().insert(id, container.clone());
        Ok(container)
    }

    pub async fn get_or_create(&self, options: ContainerOptions, name: Option<String>) -> Result<(LiteContainer, bool), String> {
        if let Some(ref n) = name {
            let guard = self.containers.lock().unwrap();
            for ctr in guard.values() {
                if ctr.name.as_ref() == Some(n) {
                    return Ok((ctr.clone(), false));
                }
            }
        }
        let ctr = self.create(options, name).await?;
        Ok((ctr, true))
    }

    pub async fn list_info(&self) -> Result<Vec<ContainerInfoCore>, String> {
        let guard = self.containers.lock().unwrap();
        Ok(guard.values().map(|c| ContainerInfoCore {
            id: c.id.clone(),
            name: c.name.clone(),
            status: c.status.clone(),
            image: c.image.clone(),
        }).collect())
    }

    pub async fn get_info(&self, id_or_name: &str) -> Result<Option<ContainerInfoCore>, String> {
        let guard = self.containers.lock().unwrap();
        for c in guard.values() {
            if c.id == id_or_name || c.name.as_deref() == Some(id_or_name) {
                return Ok(Some(ContainerInfoCore {
                    id: c.id.clone(),
                    name: c.name.clone(),
                    status: c.status.clone(),
                    image: c.image.clone(),
                }));
            }
        }
        Ok(None)
    }

    pub async fn get(&self, id_or_name: &str) -> Result<Option<LiteContainer>, String> {
        let guard = self.containers.lock().unwrap();
        for c in guard.values() {
            if c.id == id_or_name || c.name.as_deref() == Some(id_or_name) {
                return Ok(Some(c.clone()));
            }
        }
        Ok(None)
    }

    pub async fn metrics(&self) -> Result<RuntimeMetricsCore, String> {
        let guard = self.containers.lock().unwrap();
        let total = guard.len() as i64;
        let running = guard.values().filter(|c| c.status == "running").count() as i64;
        Ok(RuntimeMetricsCore {
            containers_created_total: total,
            num_running_containers: running,
        })
    }

    pub fn images(&self) -> Result<ImageHandleCore, String> {
        Ok(ImageHandleCore)
    }

    pub fn volumes(&self) -> Result<VolumeHandleCore, String> {
        Ok(VolumeHandleCore)
    }

    pub async fn remove(&self, id_or_name: &str, _force: bool) -> Result<(), String> {
        let mut guard = self.containers.lock().unwrap();
        let key = guard.iter().find(|(k, v)| k.as_str() == id_or_name || v.name.as_deref() == Some(id_or_name)).map(|(k, _)| k.clone());
        if let Some(k) = key {
            guard.remove(&k);
        }
        Ok(())
    }

    pub async fn shutdown(&self, _timeout: Option<i32>) -> Result<(), String> {
        let mut guard = self.containers.lock().unwrap();
        guard.clear();
        Ok(())
    }
}

pub fn init_logging_for(_home_dir: &str) -> Result<(), String> {
    Ok(())
}
