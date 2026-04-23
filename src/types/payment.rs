use serde::{Deserialize, Serialize};

use super::BaseResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentExpressData {
    #[serde(default)]
    pub payment_id: String,
    #[serde(default)]
    pub qr_content: String,
    #[serde(default)]
    pub method: String,
    #[serde(default)]
    pub pay_time: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub status_code: String,
    #[serde(default)]
    pub amount: f64,
    pub paid_at: Option<String>,
    #[serde(default)]
    pub created_at: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentInstantPackageLocation {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub address: String,
    #[serde(default)]
    pub phone: String,
    #[serde(default)]
    pub latitude: f64,
    #[serde(default)]
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInstantPackage {
    #[serde(default)]
    pub awb: String,
    #[serde(default)]
    pub order_id: String,
    #[serde(default)]
    pub service: String,
    #[serde(default)]
    pub service_name: String,
    #[serde(default)]
    pub status: i64,
    pub live_track_url: Option<String>,
    #[serde(default)]
    pub origin: PaymentInstantPackageLocation,
    #[serde(default)]
    pub destination: PaymentInstantPackageLocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInstantResult {
    #[serde(default)]
    pub payment_id: String,
    #[serde(default)]
    pub amount: f64,
    #[serde(default)]
    pub status_code: i64,
    pub qr_content: Option<String>,
    pub pay_time: Option<String>,
    #[serde(default)]
    pub packages: Vec<PaymentInstantPackage>,
}

/// Combined response. Inspect `data` for express payments or `result` for instant payments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPaymentResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    #[serde(default)]
    pub data: Option<PaymentExpressData>,
    #[serde(default)]
    pub result: Option<PaymentInstantResult>,
}
