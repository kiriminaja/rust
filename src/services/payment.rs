use serde_json::json;

use crate::error::{Error, Result};
use crate::http::SharedHttp;
use crate::types::GetPaymentResponse;

#[derive(Debug, Clone)]
pub struct PaymentService {
    client: SharedHttp,
}

impl PaymentService {
    pub fn new(client: SharedHttp) -> Self {
        Self { client }
    }

    pub async fn get_payment(&self, payment_id: &str) -> Result<GetPaymentResponse> {
        if payment_id.is_empty() {
            return Err(Error::InvalidArgument(
                "payment_id must not be empty".into(),
            ));
        }
        self.client
            .post_json(
                "/api/mitra/v2/get_payment",
                &json!({ "payment_id": payment_id }),
            )
            .await
    }
}
