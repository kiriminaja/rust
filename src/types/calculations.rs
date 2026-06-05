use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CalculateCODDataItem {
    pub courier_code: String,
    pub courier_service_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insurance_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CalculateCODRequest {
    pub item_price: i64,
    pub data: Vec<CalculateCODDataItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_cod: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_cod_amount_validation: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CalculateCODMessage {
    #[serde(rename = "MessageType", default)]
    pub message_type: String,
    #[serde(default)]
    pub message: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CalculateCODResult {
    #[serde(default)]
    pub billable_amount: String,
    #[serde(default)]
    pub courier_code: String,
    #[serde(default)]
    pub courier_service_code: String,
    #[serde(default)]
    pub fee: String,
    #[serde(default)]
    pub fee_percentage: f64,
    #[serde(default)]
    pub is_support_cod: bool,
    #[serde(default)]
    pub message: CalculateCODMessage,
    #[serde(default)]
    pub minimum_custom_cod: String,
    #[serde(default)]
    pub minimum_fee: String,
    #[serde(default)]
    pub tax_amount: String,
    #[serde(default)]
    pub tax_percentage: f64,
    #[serde(default)]
    pub total_fee: String,
    #[serde(default)]
    pub withdrawal_amount: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CalculateCODResponse {
    #[serde(default)]
    pub status: bool,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub method: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub results: Vec<CalculateCODResult>,
}
