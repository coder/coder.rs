use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub username: String,
    pub email: String,
    pub dotfiles_git_uri: String,
    pub roles: Vec<SiteRole>,
    pub avatar_hash: String,
    pub key_regenerated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SiteRole {
    #[serde(rename = "site-admin")]
    Admin,
    #[serde(rename = "site-auditor")]
    Auditor,
    #[serde(rename = "site-manager")]
    Manager,
    #[serde(rename = "site-member")]
    Member,
}
