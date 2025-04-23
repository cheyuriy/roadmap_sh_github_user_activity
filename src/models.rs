use serde::{Serialize, Deserialize};
use chrono::prelude::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub actor: Actor,
    pub repo: Repo,
    pub public: bool,
    pub created_at: DateTime<Utc>,
    pub org: Org,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Actor {
    pub id: i64,
    pub login: String,
    pub display_login: String,
    pub gravatar_id: String,
    pub url: String,
    pub avatar_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repo {
    pub id: i64,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Org {
    pub id: i64,
    pub login: String,
    pub gravatar_id: String,
    pub url: String,
    pub avatar_url: String,
}
