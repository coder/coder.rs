use super::*;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageId(pub String);
id_string!(ImageId);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    pub id: ImageId,
    pub organization_id: String,
    pub repository: String,
    pub description: String,
    pub url: String,
    pub default_cpu_cores: i64,
    pub default_memory_gb: i64,
    pub default_disk_gb: i64,
    pub deprecated: bool,
    pub registry: Registry,
    pub default_tag: ImageTag,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    /// All environments using this image. Must opt in to receive.
    pub environments: Option<Vec<Environment>>,
    /// IDs of all users using this image. Must opt in to receive.
    pub user_ids: Option<Vec<String>>,
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
