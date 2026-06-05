use crate::error::Result;
use crate::http::SharedHttp;
use crate::types::ProfileResponse;

#[derive(Debug, Clone)]
pub struct ProfileService {
    client: SharedHttp,
}

impl ProfileService {
    pub fn new(client: SharedHttp) -> Self {
        Self { client }
    }

    pub async fn get(&self) -> Result<ProfileResponse> {
        self.client.get_json("/api/mitra/v6.2/profile").await
    }
}
