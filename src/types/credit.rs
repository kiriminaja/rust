use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreditBalanceData {
    #[serde(default)]
    pub balance: f64,
}

/// Response shape for `GET /api/mitra/v6.2/credit/balance`. Unlike most
/// endpoints the `code` field is returned as a string here.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreditBalanceResponse {
    #[serde(default)]
    pub status: bool,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub method: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub data: CreditBalanceData,
}
