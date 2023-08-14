use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct GqlResponse<T> {
    pub data: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryResponse {
    pub categories: Vec<Category>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BeatSaberVersionResponse {
    pub beat_saber_versions: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub github_id: i32,
    pub username: String,
    pub display_name: Option<String>,
    pub email: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub banner: Option<String>,
    pub permissions: i32,
    pub api_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Instance {
    pub name: String,
    pub path: PathBuf,
    pub game_version: String,
}

impl std::fmt::Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.game_version)
    }
}