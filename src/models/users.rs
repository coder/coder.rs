use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub username: String,
    pub email: String,
    pub dotfiles_git_uri: String,
    pub roles: Vec<String>,
    pub avatar_hash: String,
    pub key_regenerated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
