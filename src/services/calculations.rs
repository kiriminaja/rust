use crate::error::Result;
use crate::http::SharedHttp;
use crate::types::{CalculateCODRequest, CalculateCODResponse};

#[derive(Debug, Clone)]
pub struct CalculationsService {
    client: SharedHttp,
}

impl CalculationsService {
    pub fn new(client: SharedHttp) -> Self {
        Self { client }
    }

    pub async fn cod(&self, payload: &CalculateCODRequest) -> Result<CalculateCODResponse> {
        self.client.post_json("/api/mitra/calculations/cod", payload).await
    }
}
