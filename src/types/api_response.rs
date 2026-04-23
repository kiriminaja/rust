use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Generic response envelope that may contain `data`, `datas`, `result`, or `results`
/// in raw JSON form. Mirrors the Go `KAResponse` type.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KAResponse {
    #[serde(default)]
    pub status: bool,
    #[serde(default)]
    pub text: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub method: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datas: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
}
