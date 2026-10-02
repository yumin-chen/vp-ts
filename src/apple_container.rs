use napi_derive::napi;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[napi(object)]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IO {
    pub stdin: Option<String>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub tty: bool,
}

#[napi(object)]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InfoRequest {
    pub id: String,
    pub query: Option<String>,
}

#[napi(object)]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InfoResponse {
    pub id: String,
    pub status: String,
    pub info: HashMap<String, String>,
}

#[napi(object)]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClientStream {
    pub stream_id: String,
    pub metadata: HashMap<String, String>,
}

#[napi]
#[derive(Debug, Clone, Default)]
pub struct BuildTransfer {
    metadata: HashMap<String, String>,
}

#[napi]
impl BuildTransfer {
    #[napi(constructor)]
    pub fn new(metadata: Option<HashMap<String, String>>) -> Self {
        Self {
            metadata: metadata.unwrap_or_default(),
        }
    }

    #[napi(getter)]
    pub fn get_metadata(&self) -> HashMap<String, String> {
        self.metadata.clone()
    }

    #[napi]
    pub fn stage(&self) -> Option<String> {
        self.metadata.get("stage").and_then(|s| if s.is_empty() { None } else { Some(s.clone()) })
    }

    #[napi]
    pub fn method(&self) -> Option<String> {
        self.metadata.get("method").and_then(|s| if s.is_empty() { None } else { Some(s.clone()) })
    }

    #[napi]
    pub fn include_patterns(&self) -> Option<Vec<String>> {
        self.metadata.get("include-patterns").and_then(|s| {
            if s.is_empty() {
                None
            } else {
                Some(s.split(',').map(|p| p.to_string()).collect())
            }
        })
    }

    #[napi]
    pub fn follow_paths(&self) -> Option<Vec<String>> {
        self.metadata.get("followpaths").and_then(|s| {
            if s.is_empty() {
                None
            } else {
                Some(s.split(',').map(|p| p.to_string()).collect())
            }
        })
    }

    #[napi]
    pub fn mode(&self) -> Option<String> {
        self.metadata.get("mode").cloned()
    }

    #[napi]
    pub fn size(&self) -> Option<i64> {
        self.metadata.get("size").and_then(|s| {
            if s.is_empty() {
                None
            } else {
                s.parse::<i64>().ok()
            }
        })
    }

    #[napi]
    pub fn offset(&self) -> Option<f64> {
        self.metadata.get("offset").and_then(|s| {
            if s.is_empty() {
                None
            } else {
                s.parse::<u64>().ok().map(|v| v as f64)
            }
        })
    }

    #[napi]
    pub fn len(&self) -> Option<i64> {
        self.metadata.get("length").and_then(|s| {
            if s.is_empty() {
                None
            } else {
                s.parse::<i64>().ok()
            }
        })
    }
}

#[napi]
#[derive(Debug, Clone, Default)]
pub struct ImageTransfer {
    metadata: HashMap<String, String>,
}

#[napi]
impl ImageTransfer {
    #[napi(constructor)]
    pub fn new(metadata: Option<HashMap<String, String>>) -> Self {
        Self {
            metadata: metadata.unwrap_or_default(),
        }
    }

    #[napi(getter)]
    pub fn get_metadata(&self) -> HashMap<String, String> {
        self.metadata.clone()
    }

    #[napi]
    pub fn stage(&self) -> Option<String> {
        self.metadata.get("stage").cloned()
    }

    #[napi]
    pub fn method(&self) -> Option<String> {
        self.metadata.get("method").cloned()
    }

    #[napi]
    pub fn ref_name(&self) -> Option<String> {
        self.metadata.get("ref").cloned()
    }

    #[napi]
    pub fn platform(&self) -> Option<String> {
        self.metadata.get("platform").cloned()
    }

    #[napi]
    pub fn mode(&self) -> Option<String> {
        self.metadata.get("mode").cloned()
    }

    #[napi]
    pub fn size(&self) -> Option<i64> {
        self.metadata.get("size").and_then(|s| s.parse::<i64>().ok())
    }

    #[napi]
    pub fn len(&self) -> Option<i64> {
        self.metadata.get("length").and_then(|s| s.parse::<i64>().ok())
    }

    #[napi]
    pub fn offset(&self) -> Option<f64> {
        self.metadata.get("offset").and_then(|s| s.parse::<u64>().ok().map(|v| v as f64))
    }
}

#[napi]
#[derive(Debug, Clone, Default)]
pub struct ServerStream {
    image_transfer: Option<ImageTransfer>,
    build_transfer: Option<BuildTransfer>,
    io: Option<IO>,
}

#[napi]
impl ServerStream {
    #[napi(constructor)]
    pub fn new(
        image_transfer: Option<&ImageTransfer>,
        build_transfer: Option<&BuildTransfer>,
        io: Option<IO>,
    ) -> Self {
        Self {
            image_transfer: image_transfer.cloned(),
            build_transfer: build_transfer.cloned(),
            io,
        }
    }

    #[napi]
    pub fn get_image_transfer(&self) -> Option<ImageTransfer> {
        self.image_transfer.clone()
    }

    #[napi]
    pub fn get_build_transfer(&self) -> Option<BuildTransfer> {
        self.build_transfer.clone()
    }

    #[napi]
    pub fn get_io(&self) -> Option<IO> {
        self.io.clone()
    }
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountConfig {
    pub source: String,
    pub target: String,
    pub is_virtiofs: bool,
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerConfig {
    pub id: String,
    pub status: String,
    pub mounts: Vec<MountConfig>,
    pub terminal: bool,
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct ContainerStartOptions {
    pub attach: bool,
    pub interactive: bool,
    pub container_id: String,
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct ContainerStartResult {
    pub success: bool,
    pub exit_code: i32,
    pub container_id: String,
    pub detached: bool,
}

#[napi]
pub fn start_container(
    container_id: String,
    attach: Option<bool>,
    interactive: Option<bool>,
    config_json: Option<String>,
) -> napi::Result<ContainerStartResult> {
    let attach = attach.unwrap_or(false);
    let interactive = interactive.unwrap_or(false);
    let detach = !attach && !interactive;

    let config: ContainerConfig = if let Some(raw) = config_json {
        serde_json::from_str(&raw).unwrap_or(ContainerConfig {
            id: container_id.clone(),
            status: "stopped".to_string(),
            mounts: vec![],
            terminal: false,
        })
    } else {
        ContainerConfig {
            id: container_id.clone(),
            status: "stopped".to_string(),
            mounts: vec![],
            terminal: false,
        }
    };

    if config.status == "running" {
        if !detach {
            return Err(napi::Error::from_reason(
                "attach is currently unsupported on already running containers",
            ));
        }
        return Ok(ContainerStartResult {
            success: true,
            exit_code: 0,
            container_id,
            detached: true,
        });
    }

    for mount in &config.mounts {
        if mount.is_virtiofs && !Path::new(&mount.source).exists() {
            return Err(napi::Error::from_reason(format!(
                "mount source path '{}' does not exist",
                mount.source
            )));
        }
    }

    Ok(ContainerStartResult {
        success: true,
        exit_code: 0,
        container_id,
        detached: detach,
    })
}
