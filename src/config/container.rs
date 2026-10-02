use napi_derive::napi;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    pub rosetta: bool,
    pub cpus: i32,
    pub memory: String,
    pub image: String,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            rosetta: true,
            cpus: 2,
            memory: "2048MB".to_string(),
            image: "ghcr.io/apple/container-builder-shim/builder:latest".to_string(),
        }
    }
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerConfig {
    pub cpus: i32,
    pub memory: String,
}

impl Default for ContainerConfig {
    fn default() -> Self {
        Self {
            cpus: 4,
            memory: "1g".to_string(),
        }
    }
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DNSConfig {
    pub domain: Option<String>,
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelConfig {
    pub binary_path: String,
    pub url: String,
    pub digest: String,
}

impl Default for KernelConfig {
    fn default() -> Self {
        Self {
            binary_path: "opt/kata/share/kata-containers/vmlinux-6.18.35-197-debug".to_string(),
            url: "https://github.com/kata-containers/kata-containers/releases/download/3.32.0/kata-static-3.32.0-arm64.tar.zst".to_string(),
            digest: "sha256:8736c054d9223974735394f822000823baef509e1c33405ec798240fa9b6e4b5".to_string(),
        }
    }
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkConfig {
    pub subnet: Option<String>,
    pub subnetv6: Option<String>,
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryConfig {
    pub domain: String,
}

impl Default for RegistryConfig {
    fn default() -> Self {
        Self {
            domain: "docker.io".to_string(),
        }
    }
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VminitConfig {
    pub image: String,
}

impl Default for VminitConfig {
    fn default() -> Self {
        Self {
            image: "ghcr.io/apple/containerization/vminit:latest".to_string(),
        }
    }
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContainerSystemConfig {
    pub build: Option<BuildConfig>,
    pub container: Option<ContainerConfig>,
    pub dns: Option<DNSConfig>,
    pub kernel: Option<KernelConfig>,
    pub network: Option<NetworkConfig>,
    pub registry: Option<RegistryConfig>,
    pub vminit: Option<VminitConfig>,
}

#[napi]
pub fn load_container_system_config(config_path: Option<String>) -> napi::Result<ContainerSystemConfig> {
    let path = if let Some(p) = config_path {
        PathBuf::from(p)
    } else {
        dirs_home().join(".config/container/config.toml")
    };

    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(cfg) = toml::from_str::<ContainerSystemConfig>(&content) {
                return Ok(cfg);
            }
        }
    }

    Ok(ContainerSystemConfig {
        build: Some(BuildConfig::default()),
        container: Some(ContainerConfig::default()),
        dns: Some(DNSConfig::default()),
        kernel: Some(KernelConfig::default()),
        network: Some(NetworkConfig::default()),
        registry: Some(RegistryConfig::default()),
        vminit: Some(VminitConfig::default()),
    })
}

fn dirs_home() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/root"))
}
