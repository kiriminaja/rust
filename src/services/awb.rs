use crate::error::Result;
use crate::http::SharedHttp;
use crate::types::{PrintAWBRequest, PrintAWBResponse};

#[derive(Debug, Clone)]
pub struct AWBService {
    client: SharedHttp,
}

impl AWBService {
    pub fn new(client: SharedHttp) -> Self {
        Self { client }
    }

    pub async fn print(&self, payload: &PrintAWBRequest) -> Result<PrintAWBResponse> {
        self.client.post_json("/api/mitra/v6.1/awb/print", payload).await
    }
}
