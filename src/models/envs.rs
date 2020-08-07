use super::Duration;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Environment {
    pub id: String,
    pub name: String,
    pub username: String,
    pub image_id: String,
    pub image_tag: String,
    pub image_digest: String,
    pub organization_id: String,
    pub user_id: String,
    pub last_built_at: DateTime<Utc>,
    pub cpu_cores: f64,
    pub memory_gb: i64,
    pub disk_gb: i64,
    pub gpus: i64,
    pub latest_stat: EnvironmentStat,
    pub updating: bool,
    pub rebuild_messages: Vec<RebuildMessage>,
    pub last_opened_at: DateTime<Utc>,
    pub last_connection_at: DateTime<Utc>,
    pub auto_off_threshold: Duration,
    pub service_ids: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RebuildMessage {
    pub text: String,
    pub required: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentStat {
    pub time: DateTime<Utc>,
    pub last_online: String,
    pub container_status: EnvironmentStatus,
    pub stat_error: String,
    pub cpu_usage: f32,
    pub memory_total: i64,
    pub memory_usage: f32,
    pub disk_total: i64,
    pub disk_used: i64,
    pub service_stat: Vec<ServiceStat>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceStat {
    pub name: String,
    pub status: EnvironmentStatus,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnvironmentStatus {
    CREATING,
    OFF,
    ON,
    FAILED,
    UNKNOWN,
}

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
