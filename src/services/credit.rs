use crate::error::Result;
use crate::http::SharedHttp;
use crate::types::CreditBalanceResponse;

#[derive(Debug, Clone)]
pub struct CreditService {
    client: SharedHttp,
}

impl CreditService {
    pub fn new(client: SharedHttp) -> Self {
        Self { client }
    }

    /// Fetches the current KiriminAja credit balance for the authenticated
    /// mitra account from `GET /api/mitra/v6.2/credit/balance`.
    pub async fn balance(&self) -> Result<CreditBalanceResponse> {
        self.client.get_json("/api/mitra/v6.2/credit/balance").await
    }
}
