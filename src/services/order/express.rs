use std::collections::HashMap;

use reqwest::Method;
use serde_json::json;

use crate::error::{Error, Result};
use crate::http::{RequestOptions, SharedHttp};
use crate::types::{
    CancelExpressOrderResponse, ExpressTrackingResponse, KAResponse, RequestPickupPayload,
};

#[derive(Debug, Clone)]
pub struct ExpressOrderService {
    client: SharedHttp,
}

impl ExpressOrderService {
    pub fn new(client: SharedHttp) -> Self {
        Self { client }
    }

    pub async fn track(&self, order_id: &str) -> Result<ExpressTrackingResponse> {
        if order_id.is_empty() {
            return Err(Error::InvalidArgument("order_id must not be empty".into()));
        }
        self.client
            .post_json("/api/mitra/tracking", &json!({ "order_id": order_id }))
            .await
    }

    /// Cancel an express shipment. KiriminAja expects `awb` and `reason` as
    /// query parameters on a POST request.
    pub async fn cancel(&self, awb: &str, reason: &str) -> Result<CancelExpressOrderResponse> {
        if awb.is_empty() {
            return Err(Error::InvalidArgument("awb must not be empty".into()));
        }
        let mut query: HashMap<&str, String> = HashMap::new();
        query.insert("awb", awb.to_string());
        query.insert("reason", reason.to_string());

        self.client
            .request_json::<_, ()>(
                "/api/mitra/v3/cancel_shipment",
                RequestOptions {
                    method: Some(Method::POST),
                    query: Some(query),
                    ..Default::default()
                },
            )
            .await
    }

    pub async fn request_pickup(&self, payload: &RequestPickupPayload) -> Result<KAResponse> {
        if payload.name.is_empty() {
            return Err(Error::InvalidArgument(
                "payload.name must not be empty".into(),
            ));
        }
        if payload.phone.is_empty() {
            return Err(Error::InvalidArgument(
                "payload.phone must not be empty".into(),
            ));
        }
        if payload.address.is_empty() {
            return Err(Error::InvalidArgument(
                "payload.address must not be empty".into(),
            ));
        }
        if payload.kecamatan_id <= 0 {
            return Err(Error::InvalidArgument(
                "payload.kecamatan_id must be greater than 0".into(),
            ));
        }
        if payload.schedule.is_empty() {
            return Err(Error::InvalidArgument(
                "payload.schedule must not be empty".into(),
            ));
        }
        if payload.packages.is_empty() {
            return Err(Error::InvalidArgument(
                "payload.packages must not be empty".into(),
            ));
        }
        self.client
            .post_json("/api/mitra/v6.1/request_pickup", payload)
            .await
    }
}
