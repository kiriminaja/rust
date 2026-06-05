//! Shared response wrappers and domain types mirroring the Go `types` package.

mod address;
mod api_response;
mod awb;
mod calculations;
mod courier;
mod coverage_area;
mod credit;
mod order;
mod payment;
mod pickup;
mod profile;

pub use address::*;
pub use api_response::*;
pub use awb::*;
pub use calculations::*;
pub use courier::*;
pub use coverage_area::*;
pub use credit::*;
pub use order::*;
pub use payment::*;
pub use pickup::*;
pub use profile::*;

use serde::{Deserialize, Serialize};

/// Common envelope returned by most KiriminAja endpoints.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BaseResponse {
    #[serde(default)]
    pub status: bool,
    #[serde(default)]
    pub method: String,
    #[serde(default)]
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
}
