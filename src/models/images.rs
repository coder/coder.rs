use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::Environment;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    pub id: String,
    pub organization_id: String,
    pub repository: String,
    pub description: String,
    pub url: String,
    pub default_cpu_cores: i64,
    pub default_memory_gb: i64,
    pub default_disk_gb: i64,
    pub deprecated: bool,
    pub environments: Vec<Environment>,
    pub registry: Registry,
    pub default_tag: ImageTag,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Registry {
    pub id: String,
    pub organization_id: String,
    pub friendly_name: String,
    pub registry: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageTag {
    pub image_id: String,
    pub tag: String,
    pub latest_hash: String,
    pub hash_last_updated_at: DateTime<Utc>,
    pub environments: Vec<Environment>,
    pub os_release: OsRelease,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OsRelease {
    pub id: String,
    pub pretty_name: String,
    pub home_url: String,
}
