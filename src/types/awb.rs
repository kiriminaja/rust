use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PrintAWBRequest {
    pub awb: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PrintAWBData {
    #[serde(default)]
    pub url: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PrintAWBResult {
    #[serde(default)]
    pub data: PrintAWBData,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PrintAWBResponse {
    #[serde(default)]
    pub status: bool,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub method: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub data: PrintAWBResult,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<serde_json::Value>,
}
