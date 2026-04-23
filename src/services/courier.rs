use serde_json::json;

use crate::error::{Error, Result};
use crate::http::SharedHttp;
use crate::types::{
    CourierDetailResponse, CourierGroupResponse, CourierListResponse, SetCourierPreferenceResponse,
};

#[derive(Debug, Clone)]
pub struct CourierService {
    pub(crate) client: SharedHttp,
}

impl CourierService {
    pub fn new(client: SharedHttp) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<CourierListResponse> {
        self.client.post_empty("/api/mitra/couriers").await
    }

    pub async fn group(&self) -> Result<CourierGroupResponse> {
        self.client.post_empty("/api/mitra/couriers_group").await
    }

    pub async fn detail(&self, courier_code: &str) -> Result<CourierDetailResponse> {
        if courier_code.is_empty() {
            return Err(Error::InvalidArgument(
                "courier_code must not be empty".into(),
            ));
        }
        self.client
            .post_json(
                "/api/mitra/courier_services",
                &json!({ "courier_code": courier_code }),
            )
            .await
    }

    pub async fn set_whitelist_services(
        &self,
        services: &[String],
    ) -> Result<SetCourierPreferenceResponse> {
        if services.is_empty() {
            return Err(Error::InvalidArgument("services must not be empty".into()));
        }
        self.client
            .post_json(
                "/api/mitra/v3/set_whitelist_services",
                &json!({ "services": services }),
            )
            .await
    }
}
