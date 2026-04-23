use serde_json::json;

use crate::error::{Error, Result};
use crate::http::SharedHttp;
use crate::types::{
    CancelInstantOrderResponse, CreateInstantPickupResponse, FindNewInstantDriverResponse,
    InstantPickupPayload, InstantTrackingResponse,
};

#[derive(Debug, Clone)]
pub struct InstantOrderService {
    client: SharedHttp,
}

impl InstantOrderService {
    pub fn new(client: SharedHttp) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        payload: &InstantPickupPayload,
    ) -> Result<CreateInstantPickupResponse> {
        if payload.service_type.is_empty() {
            return Err(Error::InvalidArgument(
                "payload.service_type must not be empty".into(),
            ));
        }
        if payload.packages.is_empty() {
            return Err(Error::InvalidArgument(
                "payload.packages must not be empty".into(),
            ));
        }
        self.client
            .post_json("/api/mitra/v4/instant/pickup/request", payload)
            .await
    }

    pub async fn track(&self, order_id: &str) -> Result<InstantTrackingResponse> {
        if order_id.is_empty() {
            return Err(Error::InvalidArgument("order_id must not be empty".into()));
        }
        self.client
            .get_json(&format!("/api/mitra/v4/instant/tracking/{}", order_id))
            .await
    }

    pub async fn cancel(&self, order_id: &str) -> Result<CancelInstantOrderResponse> {
        if order_id.is_empty() {
            return Err(Error::InvalidArgument("order_id must not be empty".into()));
        }
        self.client
            .delete_json(&format!("/api/mitra/v4/instant/pickup/void/{}", order_id))
            .await
    }

    pub async fn find_new_driver(
        &self,
        order_id: &str,
    ) -> Result<FindNewInstantDriverResponse> {
        if order_id.is_empty() {
            return Err(Error::InvalidArgument("order_id must not be empty".into()));
        }
        self.client
            .post_json(
                "/api/mitra/v4/instant/pickup/find-new-driver",
                &json!({ "order_id": order_id }),
            )
            .await
    }
}
