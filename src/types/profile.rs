use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProfileMeta {
    #[serde(default)]
    pub has_pin: bool,
    #[serde(default)]
    pub payment_method: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProfileData {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub metadata: ProfileMeta,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProfileResponse {
    #[serde(default)]
    pub status: bool,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub method: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub results: ProfileData,
}
