use serde::{Deserialize, Serialize};

use super::BaseResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourierListItem {
    pub code: String,
    pub name: String,
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourierGroupItem {
    pub code: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourierServiceItem {
    pub name: String,
    pub code: String,
    pub cut_off_time: Option<String>,
    #[serde(rename = "volumetrik")]
    pub volumetric: Option<String>,
    pub rounded: Option<i64>,
    #[serde(default)]
    pub courier_group: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourierListResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    #[serde(default)]
    pub datas: Vec<CourierListItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourierGroupResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    #[serde(default)]
    pub datas: Vec<CourierGroupItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourierDetailResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    #[serde(default)]
    pub datas: Vec<CourierServiceItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCourierPreferenceResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
}
