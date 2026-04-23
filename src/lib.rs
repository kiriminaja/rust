//! # KiriminAja Rust SDK
//!
//! Async Rust client for the [KiriminAja](https://kiriminaja.com) logistics API.
//! API surface mirrors the official [Go SDK](https://github.com/kiriminaja/go).
//!
//! ## Quick start
//!
//! ```no_run
//! use kiriminaja::{Client, Config, Env};
//!
//! # async fn run() -> kiriminaja::Result<()> {
//! let client = Client::new(Config {
//!     env: Env::Sandbox,
//!     api_key: std::env::var("KIRIMINAJA_API_KEY").unwrap_or_default(),
//!     ..Default::default()
//! });
//!
//! let provinces = client.address.provinces().await?;
//! println!("{:?}", provinces);
//! # Ok(()) }
//! ```

pub mod config;
pub mod error;
pub mod http;
pub mod services;
pub mod types;

#[cfg(feature = "blocking")]
pub mod blocking;

use std::sync::Arc;

pub use config::{Config, Env};
pub use error::{Error, Result};
pub use http::HttpClient;

use services::address::AddressService;
use services::courier::CourierService;
use services::coverage_area::CoverageAreaService;
use services::credit::CreditService;
use services::order::OrderService;
use services::payment::PaymentService;
use services::pickup::PickupService;

/// Top-level KiriminAja client. Mirrors the Go `Client` struct: services are
/// exposed as public fields so callers write `client.address.provinces()` etc.
#[derive(Debug, Clone)]
pub struct Client {
    pub address: AddressService,
    pub courier: CourierService,
    pub coverage_area: CoverageAreaService,
    pub credit: CreditService,
    pub order: OrderService,
    pub payment: PaymentService,
    pub pickup: PickupService,

    #[allow(dead_code)]
    http: Arc<HttpClient>,
}

impl Client {
    pub fn new(cfg: Config) -> Self {
        let http = Arc::new(HttpClient::from_config(&cfg));
        Self {
            address: AddressService::new(http.clone()),
            courier: CourierService::new(http.clone()),
            coverage_area: CoverageAreaService::new(http.clone()),
            credit: CreditService::new(http.clone()),
            order: OrderService::new(http.clone()),
            payment: PaymentService::new(http.clone()),
            pickup: PickupService::new(http.clone()),
            http,
        }
    }
}
