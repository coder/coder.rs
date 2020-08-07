use super::ContainerStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Service {
    pub id: String,
    pub name: String,
    pub description: String,
    pub image_id: String,
    pub image_tag: String,
    pub command: String,
    pub args: Vec<String>,
    pub privileged: bool,
    pub volume_mounts: Vec<ServiceVolumeMount>,
    pub env_vars: Vec<ServiceEnvVar>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceVolumeMount {
    pub name: String,
    pub service_id: String,
    pub path: String,
    pub size_gb: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceEnvVar {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceStat {
    pub name: String,
    pub status: ContainerStatus,
    pub reason: String,
}
