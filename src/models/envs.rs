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

/// Duration is a wrapper around chrono::Duration that Serializes into and Deserializes from
/// millisecond precision integers.
#[derive(Debug, Clone, PartialEq)]
pub struct Duration(chrono::Duration);

// Allow Duration to be used as a chrono::Duration.
impl std::ops::Deref for Duration {
    type Target = chrono::Duration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

use serde::Deserializer;
impl<'de> Deserialize<'de> for Duration {
    fn deserialize<D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let u = i64::deserialize(deserializer)?;
        Ok(Duration(chrono::Duration::milliseconds(u)))
    }
}

use serde::Serializer;
impl Serialize for Duration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(self.0.num_milliseconds())
    }
}

#[cfg(test)]
mod test {
    use super::Duration;
    use serde_json;

    #[test]
    fn test_serialize_duration() {
        let ms = 86400000i64;
        let d = Duration(chrono::Duration::milliseconds(ms));
        assert_eq!(serde_json::to_string(&d).unwrap(), ms.to_string());
    }

    #[test]
    fn test_deserialize_duration() {
        let ms = 86400000i64;
        let d: Duration = serde_json::from_str(&ms.to_string()).unwrap();
        assert_eq!(d.num_milliseconds(), ms);
    }
}
