pub mod apple_container;
pub mod cli;
pub mod config;

pub use apple_container::*;
pub use cli::*;
pub use config::{
    load_container_system_config, BuildConfig, ContainerSystemConfig, DNSConfig, KernelConfig,
    NetworkConfig, RegistryConfig, VminitConfig,
};

use napi_derive::napi;
use std::collections::HashMap;

#[napi]
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct GetOrCreateResult {
    pub container_id: String,
    pub created: bool,
    pub status: String,
}

#[napi]
pub struct Container {
    id: String,
    status: String,
    metadata: HashMap<String, String>,
}

#[napi]
impl Container {
    #[napi(constructor)]
    pub fn new(id: String, status: Option<String>) -> Self {
        Self {
            id,
            status: status.unwrap_or_else(|| "stopped".to_string()),
            metadata: HashMap::new(),
        }
    }

    #[napi]
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    #[napi]
    pub fn get_status(&self) -> String {
        self.status.clone()
    }

    #[napi]
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    #[napi]
    pub fn get_metadata(&self, key: String) -> Option<String> {
        self.metadata.get(&key).cloned()
    }
}

#[napi]
pub fn get_or_create_container(id: String) -> GetOrCreateResult {
    GetOrCreateResult {
        container_id: id,
        created: true,
        status: "created".to_string(),
    }
}
