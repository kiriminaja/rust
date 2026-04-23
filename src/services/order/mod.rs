pub mod express;
pub mod instant;

use crate::http::SharedHttp;

use express::ExpressOrderService;
use instant::InstantOrderService;

#[derive(Debug, Clone)]
pub struct OrderService {
    pub express: ExpressOrderService,
    pub instant: InstantOrderService,
}

impl OrderService {
    pub fn new(client: SharedHttp) -> Self {
        Self {
            express: ExpressOrderService::new(client.clone()),
            instant: InstantOrderService::new(client),
        }
    }
}
