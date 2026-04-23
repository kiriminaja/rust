use crate::error::Result;
use crate::http::SharedHttp;
use crate::types::PickupSchedulesResponse;

#[derive(Debug, Clone)]
pub struct PickupService {
    client: SharedHttp,
}

impl PickupService {
    pub fn new(client: SharedHttp) -> Self {
        Self { client }
    }

    pub async fn schedules(&self) -> Result<PickupSchedulesResponse> {
        self.client.post_empty("/api/mitra/v2/schedules").await
    }
}
