use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Organization {
    pub id: String,
    pub name: String,
    pub description: String,
    pub default: bool,
    pub members: Vec<OrgMember>,
    pub environment_count: i64,
    pub resource_namespace: String,
    pub auto_off_threshold: i64,
    pub cpu_provisioning_rate: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrgMember {
    #[serde(flatten)]
    pub user: super::User,

    pub organization_roles: Vec<OrgRole>,
    pub has_active_environments: bool,
    pub joined_at: DateTime<Utc>,
    pub roles_updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrgRole {
    #[serde(rename = "organization-admin")]
    Admin,
    #[serde(rename = "organization-manager")]
    Manager,
    #[serde(rename = "registry-manager")]
    RegistryManager,
    #[serde(rename = "organization-member")]
    Member,
}
